# BCO Training Interface

A Gradio-based interface for training models using Best-of-N Contrastive Optimization (BCO).

## Features

- 🚀 **Easy Training**: Simple web interface for BCO model training
- 🤖 **Model Support**: Compatible with Hugging Face models (optimized for Gemma3)
- 📊 **Real-time Monitoring**: Live progress tracking and logs
- ⚙️ **Configurable**: Adjustable hyperparameters
- 🔧 **Zero GPU Optimized**: Designed for Hugging Face Spaces

## Usage

1. **Model Name**: Enter a Hugging Face model name (e.g., `google/gemma-3-270m-it`)
2. **Data File**: Upload or specify path to your JSON training data
3. **Configure Parameters**: Adjust beta, learning rate, epochs, etc.
4. **Start Training**: Click "Start Training" to begin

## Data Format

Your JSON data should be in the following format:

```json
[
  {
    "prompt": "Your prompt text here",
    "rejected": "Rejected response text",
    "chosen": "Chosen response text"
  }
]
```

## Parameters

- **Beta**: Controls contrastive learning weight (0.01-1.0)
- **Learning Rate**: Training learning rate (1e-6 to 1e-3)
- **Epochs**: Number of training epochs (1-10)
- **Max Length**: Maximum sequence length (256-2048)
- **Batch Size**: Training batch size (1-8)

## Technical Details

- Uses `eager` attention implementation for Gemma3 models
- Optimized for zero GPU environments
- Real-time progress tracking
- Automatic model loading and configuration

## Files

- `bco_gradio.py`: Main Gradio interface
- `requirements.txt`: Python dependencies
- `todozi_bco_fixed.json`: Sample training data

## Deployment

This interface is designed to run on Hugging Face Spaces with zero GPU. Simply upload the files and run `python bco_gradio.py`.
