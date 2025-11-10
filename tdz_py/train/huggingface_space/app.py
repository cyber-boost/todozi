#!/usr/bin/env python3
"""
Simplified Gradio interface for BCO training on Hugging Face Spaces.
"""

import gradio as gr
import torch
from transformers import AutoModelForCausalLM, AutoTokenizer
from trl import BCOTrainer, BCOConfig
import json
from datasets import load_dataset
import os
import tempfile
import threading
import time

# Global training state
training_state = {
    "running": False,
    "progress": 0,
    "logs": [],
    "trainer": None
}

def load_model_with_eager_attention(model_name):
    """Load model with eager attention implementation."""
    try:
        print(f"Loading model: {model_name}")
        
        # Load tokenizer
        tokenizer = AutoTokenizer.from_pretrained(model_name)
        
        # Load model with eager attention
        model = AutoModelForCausalLM.from_pretrained(
            model_name,
            attn_implementation="eager",
            torch_dtype=torch.float16 if torch.cuda.is_available() else torch.float32,
            device_map="auto" if torch.cuda.is_available() else None,
            low_cpu_mem_usage=True
        )
        
        # Load reference model
        ref_model = AutoModelForCausalLM.from_pretrained(
            model_name,
            attn_implementation="eager",
            torch_dtype=torch.float16 if torch.cuda.is_available() else torch.float32,
            device_map="auto" if torch.cuda.is_available() else None,
            low_cpu_mem_usage=True
        )
        
        return model, ref_model, tokenizer
    except Exception as e:
        raise Exception(f"Failed to load model: {str(e)}")

def prepare_training_data(data_file):
    """Prepare training data from file."""
    try:
        if not os.path.exists(data_file):
            raise Exception(f"Data file not found: {data_file}")
        
        dataset = load_dataset('json', data_files=data_file)
        return dataset['train']
    except Exception as e:
        raise Exception(f"Failed to load data: {str(e)}")

def train_model(model_name, data_file, beta, lr, epochs, max_len, batch_size):
    """Train the BCO model."""
    global training_state
    
    try:
        training_state["running"] = True
        training_state["progress"] = 0
        training_state["logs"] = []
        
        # Step 1: Load model
        training_state["logs"].append("🤖 Loading model...")
        model, ref_model, tokenizer = load_model_with_eager_attention(model_name)
        training_state["progress"] = 20
        
        # Step 2: Load data
        training_state["logs"].append("📊 Loading training data...")
        train_dataset = prepare_training_data(data_file)
        training_state["logs"].append(f"📊 Loaded {len(train_dataset)} examples")
        training_state["progress"] = 40
        
        # Step 3: Configure BCO
        training_state["logs"].append("⚙️ Configuring BCO...")
        bco_config = BCOConfig(
            beta=beta,
            max_length=max_len,
            max_prompt_length=max_len // 2,
            per_device_train_batch_size=batch_size,
            per_device_eval_batch_size=batch_size,
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
        training_state["progress"] = 60
        
        # Step 4: Initialize trainer
        training_state["logs"].append("🚀 Initializing BCO Trainer...")
        trainer = BCOTrainer(
            model=model,
            ref_model=ref_model,
            args=bco_config,
            processing_class=tokenizer,
            train_dataset=train_dataset,
        )
        training_state["trainer"] = trainer
        training_state["progress"] = 80
        
        # Step 5: Start training
        training_state["logs"].append("🏋️ Starting training...")
        trainer.train()
        
        training_state["progress"] = 100
        training_state["logs"].append("✅ Training completed successfully!")
        training_state["running"] = False
        
        return "✅ Training completed successfully!"
        
    except Exception as e:
        training_state["running"] = False
        training_state["logs"].append(f"❌ Error: {str(e)}")
        return f"❌ Training failed: {str(e)}"

def get_status():
    """Get current training status."""
    global training_state
    
    progress = training_state["progress"]
    logs = "\n".join(training_state["logs"][-5:])  # Last 5 logs
    
    return progress, logs

def stop_training():
    """Stop current training."""
    global training_state
    
    if training_state["running"]:
        training_state["running"] = False
        training_state["logs"].append("🛑 Training stopped by user")
        return "🛑 Training stopped"
    else:
        return "No training running"

# Create Gradio interface
with gr.Blocks(title="BCO Training", theme=gr.themes.Soft()) as demo:
    gr.Markdown("# 🚀 BCO Training Interface")
    gr.Markdown("Train models using Best-of-N Contrastive Optimization")
    
    with gr.Row():
        with gr.Column():
            gr.Markdown("## ⚙️ Configuration")
            
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
            epochs = gr.Slider(1, 5, 1, label="Epochs", step=1)
            max_len = gr.Slider(256, 1024, 512, label="Max Length", step=128)
            batch_size = gr.Slider(1, 4, 1, label="Batch Size", step=1)
            
            with gr.Row():
                train_btn = gr.Button("🚀 Start Training", variant="primary")
                stop_btn = gr.Button("🛑 Stop", variant="stop")
        
        with gr.Column():
            gr.Markdown("## 📊 Status")
            
            status = gr.Textbox(label="Status", value="Ready")
            progress = gr.Progress()
            logs = gr.Textbox(label="Logs", lines=8, interactive=False)
    
    # Event handlers
    train_btn.click(
        fn=train_model,
        inputs=[model_name, data_file, beta, lr, epochs, max_len, batch_size],
        outputs=[status]
    )
    
    stop_btn.click(fn=stop_training, outputs=[status])
    
    # Auto-refresh - removed 'every' parameter as it's not supported in this Gradio version
    # demo.load(fn=get_status, outputs=[progress, logs], every=2)
    
    gr.Markdown("""
    ## 📝 Instructions
    
    1. Enter model name (e.g., `google/gemma-3-270m-it`)
    2. Specify data file path
    3. Adjust parameters as needed
    4. Click "Start Training"
    
    **Note**: This interface uses eager attention for Gemma3 models as recommended.
    """)

if __name__ == "__main__":
    demo.launch(
        server_name="0.0.0.0",
        server_port=7860,
        share=True
    )
