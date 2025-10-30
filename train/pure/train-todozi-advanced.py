import torch
from transformers import AutoModelForCausalLM, AutoTokenizer
from trl import BCOTrainer, BCOConfig
import json
from datasets import load_dataset
import os
import glob
from pathlib import Path

"""
Batch training configuration
Select which folder to train from: "balanced", "basic", "bootstrap", "custom", "kfold", 
                                   "length", "ml", "progressive", "stratified", "todozi"

Recommended Models: 
- HuggingFaceTB/SmolLM3-3B
- openai/gpt-oss-20b
- Qwen/Qwen3-VL-2B-Thinking
- google/gemma-3-270m-it
- meta-llama/Llama-3.1-8B-Instruct
"""

# ============ CONFIGURATION ============
batch_folder = "balanced"  # Change this to select different folder
model_name = "HuggingFaceTB/SmolLM3-3B"
skip_metadata = True  # Skip files with "metadata" in the name
# =======================================

def get_batch_files(folder_name):
    """Get all JSON files from the specified batch folder"""
    pattern = f"train/batches/{folder_name}/*.json"
    files = sorted(glob.glob(pattern))
    
    if skip_metadata:
        files = [f for f in files if "metadata" not in f]
    
    return files

def load_model_and_tokenizer(model_name):
    """Load and return model and tokenizer"""
    print(f"🤖 Loading model: {model_name}")
    tokenizer = AutoTokenizer.from_pretrained(model_name)
    model = AutoModelForCausalLM.from_pretrained(model_name)
    ref_model = AutoModelForCausalLM.from_pretrained(model_name)
    print("✅ Models loaded successfully")
    return model, ref_model, tokenizer

def train_on_file(json_file, model, ref_model, tokenizer, file_index, total_files, output_dir):
    """Train on a single batch file"""
    print(f"\n{'='*80}")
    print(f"📂 File {file_index + 1}/{total_files}: {os.path.basename(json_file)}")
    print(f"{'='*80}")
    
    if not os.path.exists(json_file):
        print(f"❌ File not found: {json_file}")
        raise FileNotFoundError(f"Batch file not found: {json_file}")
    
    print("📊 Loading dataset...")
    dataset = load_dataset('json', data_files=json_file)
    print(f"📊 Dataset loaded: {len(dataset['train'])} examples")
    
    # Create output directory for this specific file
    file_name = Path(json_file).stem
    file_output_dir = f"{output_dir}/{file_name}"
    
    # BCO Configuration
    bco_config = BCOConfig(
        beta=0.1,
        max_length=1024,
        max_prompt_length=512,
        per_device_train_batch_size=1,
        per_device_eval_batch_size=1,
        num_train_epochs=1,
        learning_rate=1e-5,
        save_steps=100,
        eval_strategy="no",
        output_dir=file_output_dir,
        report_to="none",
        fp16=False,
        bf16=True,
    )
    
    print("🚀 Initializing BCO Trainer...")
    bco_trainer = BCOTrainer(
        model=model,
        ref_model=ref_model,
        args=bco_config,
        processing_class=tokenizer,
        train_dataset=dataset['train'],
    )
    print("✅ BCO Trainer initialized")
    
    print("🏋️  Starting training...")
    try:
        bco_trainer.train()
        print(f"✅ Training completed successfully for {os.path.basename(json_file)}!")
        return True
    except Exception as e:
        print(f"❌ Training failed for {os.path.basename(json_file)}: {e}")
        raise

# Get all batch files from the specified folder
batch_files = get_batch_files(batch_folder)
total_files = len(batch_files)

if total_files == 0:
    print(f"❌ No files found in folder: train/batches/{batch_folder}/")
    raise FileNotFoundError(f"No batch files found in folder: {batch_folder}")

print(f"📁 Found {total_files} files in folder: {batch_folder}")
for i, f in enumerate(batch_files):
    print(f"  {i+1}. {os.path.basename(f)}")

# Load models once for all files
model, ref_model, tokenizer = load_model_and_tokenizer(model_name)

# Train on each file
success_count = 0
for i, json_file in enumerate(batch_files):
    try:
        train_on_file(
            json_file, 
            model, 
            ref_model, 
            tokenizer, 
            i, 
            total_files,
            output_dir="./bco_output"
        )
        success_count += 1
        print(f"\n✓ Progress: {success_count}/{total_files} files completed")
    except Exception as e:
        print(f"\n✗ Failed on file {i+1}/{total_files}: {e}")
        # Continue with next file instead of stopping
        continue

print(f"\n{'='*80}")
print(f"🎉 Training Summary: {success_count}/{total_files} files completed successfully")
print(f"{'='*80}")
