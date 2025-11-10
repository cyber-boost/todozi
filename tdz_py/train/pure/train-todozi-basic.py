import torch
from transformers import AutoModelForCausalLM, AutoTokenizer
from trl import BCOTrainer, BCOConfig
import json
from datasets import load_dataset
import os

"""
batch_src - Array of all batch files in batches/ directory
Format: batches/[folder]/[file].json
"""

batch_src = [
    "train/batches/balanced/batch_balanced_prompt_10each_20251027_215120.json",
    "train/batches/balanced/batch_clustered_10clusters_20251027_215120.json",
    "train/batches/basic/batch_full_20251027_215112.json",
    "train/batches/basic/batch_half_20251027_215112.json",
    "train/batches/basic/batch_large_20251027_215112.json",
    "train/batches/basic/batch_metadata_20251027_215112.json",
    "train/batches/basic/batch_micro_20251027_215112.json",
    "train/batches/basic/batch_quarter_20251027_215112.json",
    "train/batches/basic/batch_small_20251027_215112.json",
    "train/batches/basic/batch_three_quarters_20251027_215112.json",
    "train/batches/basic/batch_tiny_20251027_215112.json",
    "train/batches/bootstrap/batch_bootstrap_sample1_20251027_215116.json",
    "train/batches/bootstrap/batch_bootstrap_sample2_20251027_215116.json",
    "train/batches/bootstrap/batch_bootstrap_sample3_20251027_215116.json",
    "train/batches/bootstrap/batch_bootstrap_sample4_20251027_215116.json",
    "train/batches/bootstrap/batch_bootstrap_sample5_20251027_215116.json",
    "train/batches/custom/batch_10%_sample1_20251027_215114.json",
    "train/batches/custom/batch_10%_sample2_20251027_215114.json",
    "train/batches/custom/batch_10%_sample3_20251027_215114.json",
    "train/batches/custom/batch_1000_sample1_20251027_215114.json",
    "train/batches/custom/batch_1000_sample2_20251027_215114.json",
    "train/batches/custom/batch_1000_sample3_20251027_215114.json",
    "train/batches/custom/batch_100_sample1_20251027_215114.json",
    "train/batches/custom/batch_100_sample2_20251027_215114.json",
    "train/batches/custom/batch_100_sample3_20251027_215114.json",
    "train/batches/custom/batch_20%_sample1_20251027_215114.json",
    "train/batches/custom/batch_5000_sample1_20251027_215114.json",
    "train/batches/custom/batch_5000_sample2_20251027_215114.json",
    "train/batches/custom/batch_5000_sample3_20251027_215114.json",
    "train/batches/custom/batch_500_sample1_20251027_215114.json",
    "train/batches/custom/batch_500_sample2_20251027_215114.json",
    "train/batches/custom/batch_500_sample3_20251027_215114.json",
    "train/batches/custom/custom_batch_metadata_20251027_215114.json",
    "train/batches/kfold/fold_1_of_5_20251027_215115.json",
    "train/batches/kfold/fold_2_of_5_20251027_215115.json",
    "train/batches/kfold/fold_3_of_5_20251027_215115.json",
    "train/batches/kfold/fold_4_of_5_20251027_215115.json",
    "train/batches/kfold/fold_5_of_5_20251027_215115.json",
    "train/batches/kfold/kfold_metadata_20251027_215115.json",
    "train/batches/length/batch_length_q100_range_249_3347_20251027_215120.json",
    "train/batches/length/batch_length_q25_range_55_167_20251027_215120.json",
    "train/batches/length/batch_length_q50_range_167_203_20251027_215120.json",
    "train/batches/length/batch_length_q75_range_203_249_20251027_215120.json",
    "train/batches/ml/batch_difficulty_easy_20251027_215119.json",
    "train/batches/ml/batch_difficulty_hard_20251027_215119.json",
    "train/batches/ml/batch_difficulty_medium_20251027_215119.json",
    "train/batches/ml/batch_diverse_1000_20251027_215119.json",
    "train/batches/ml/batch_test_15pct_20251027_215119.json",
    "train/batches/ml/batch_train_70pct_20251027_215119.json",
    "train/batches/ml/batch_validation_15pct_20251027_215119.json",
    "train/batches/progressive/batch_progressive_100pct_20251027_215115.json",
    "train/batches/progressive/batch_progressive_10pct_20251027_215115.json",
    "train/batches/progressive/batch_progressive_25pct_20251027_215115.json",
    "train/batches/progressive/batch_progressive_50pct_20251027_215115.json",
    "train/batches/progressive/batch_progressive_75pct_20251027_215115.json",
    "train/batches/stratified/batch_stratified_half_20251027_215114.json",
    "train/batches/stratified/batch_stratified_quarter_20251027_215114.json",
    "train/batches/stratified/batch_stratified_three_quarters_20251027_215114.json",
    "train/batches/todozi/full-toodzi.json",
    "train/batches/todozi/how-to-todozi-basic.json",
    "train/batches/todozi/small.json",
]

# Select the second file (index 1) from the array
json_file = batch_src[1]
if not os.path.exists(json_file):
    print(f"❌ File not found: {json_file}")
    raise FileNotFoundError(f"Batch file not found: {json_file}")
else:
    print(f"✅ Using batch file: {json_file}")

print("📊 Loading dataset...")
dataset = load_dataset('json', data_files=json_file)
print(f"📊 Dataset loaded: {len(dataset['train'])} examples")

model_name = "google/gemma-3-270m-it"
print(f"🤖 Loading model: {model_name}")
tokenizer = AutoTokenizer.from_pretrained(model_name)
model = AutoModelForCausalLM.from_pretrained(model_name)
ref_model = AutoModelForCausalLM.from_pretrained(model_name)
print("✅ Models loaded successfully")

# BCO Configuration
bco_config = BCOConfig(
    beta=0.1,  # Controls deviation from reference model
    max_length=1024,
    max_prompt_length=512,
    per_device_train_batch_size=1,
    per_device_eval_batch_size=1,
    num_train_epochs=1,
    learning_rate=1e-5,
    save_steps=100,
    eval_strategy="no",
    output_dir="./bco_output",
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

print("🏋️ Starting training...")
try:
    bco_trainer.train()
    print("✅ Training completed successfully!")
except Exception as e:
    print(f"❌ Training failed: {e}")
    raise
