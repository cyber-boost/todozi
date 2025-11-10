#!/usr/bin/env python3
"""
Minimal Gradio interface for BCO training - maximum compatibility.
"""

import gradio as gr
import torch
from transformers import AutoModelForCausalLM, AutoTokenizer
from trl import BCOTrainer, BCOConfig
from datasets import load_dataset
import os

def train_bco(model_name, data_file, beta, lr, epochs):
    """Simple BCO training function."""
    try:
        # Load model with eager attention
        print(f"Loading model: {model_name}")
        tokenizer = AutoTokenizer.from_pretrained(model_name)
        model = AutoModelForCausalLM.from_pretrained(
            model_name,
            attn_implementation="eager",
            torch_dtype=torch.float16 if torch.cuda.is_available() else torch.float32,
            device_map="auto" if torch.cuda.is_available() else None,
            low_cpu_mem_usage=True
        )
        ref_model = AutoModelForCausalLM.from_pretrained(
            model_name,
            attn_implementation="eager",
            torch_dtype=torch.float16 if torch.cuda.is_available() else torch.float32,
            device_map="auto" if torch.cuda.is_available() else None,
            low_cpu_mem_usage=True
        )
        
        # Load data
        print(f"Loading data: {data_file}")
        dataset = load_dataset('json', data_files=data_file)
        train_dataset = dataset['train']
        
        # Configure BCO
        bco_config = BCOConfig(
            beta=beta,
            max_length=512,
            max_prompt_length=256,
            per_device_train_batch_size=1,
            per_device_eval_batch_size=1,
            num_train_epochs=epochs,
            learning_rate=lr,
            save_steps=50,
            eval_strategy="no",
            output_dir="./bco_output",
            report_to="none",
            fp16=torch.cuda.is_available(),
            bf16=not torch.cuda.is_available(),
            remove_unused_columns=False,
        )
        
        # Initialize trainer
        trainer = BCOTrainer(
            model=model,
            ref_model=ref_model,
            args=bco_config,
            processing_class=tokenizer,
            train_dataset=train_dataset,
        )
        
        # Train
        print("Starting training...")
        trainer.train()
        
        return "✅ Training completed successfully!"
        
    except Exception as e:
        return f"❌ Training failed: {str(e)}"

# Create simple interface
with gr.Blocks() as demo:
    gr.Markdown("# 🚀 BCO Training Interface")
    
    with gr.Row():
        with gr.Column():
            model_name = gr.Textbox(
                label="Model Name",
                value="google/gemma-3-270m-it",
                placeholder="Hugging Face model name"
            )
            
            data_file = gr.Textbox(
                label="Data File",
                value="todozi_bco_fixed.json",
                placeholder="Path to JSON data file"
            )
            
            beta = gr.Slider(0.01, 1.0, 0.1, label="Beta")
            lr = gr.Slider(1e-6, 1e-3, 1e-5, label="Learning Rate")
            epochs = gr.Slider(1, 3, 1, label="Epochs", step=1)
            
            train_btn = gr.Button("🚀 Start Training", variant="primary")
        
        with gr.Column():
            output = gr.Textbox(label="Output", lines=10)
    
    train_btn.click(
        fn=train_bco,
        inputs=[model_name, data_file, beta, lr, epochs],
        outputs=[output]
    )
    
    gr.Markdown("""
    ## 📝 Instructions
    
    1. Enter model name (e.g., `google/gemma-3-270m-it`)
    2. Specify data file path
    3. Adjust parameters
    4. Click "Start Training"
    
    **Note**: Uses eager attention for Gemma3 models.
    """)

if __name__ == "__main__":
    demo.launch(
        server_name="0.0.0.0",
        server_port=7860,
        share=True
    )
