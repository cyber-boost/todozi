// Simple test for BCO trainer components
const { BCODataLoader, BCOConfig, BCOTrainer } = require('../bco');

async function testBCO() {
  try {
    console.log("Testing BCO components...");

    // Test data loading
    const loader = new BCODataLoader("./train/datasets/todozi/small.json");
    const { trainData, evalData } = loader.load().getDatasets();

    console.log("Dataset loaded successfully");
    console.log("Train data length:", trainData.length);
    console.log("Sample entry:", JSON.stringify(trainData[0], null, 2));

    // Test config
    const config = new BCOConfig({
      per_device_train_batch_size: 2,
      num_train_epochs: 1,
      learning_rate: 5e-5
    });

    console.log("Config created:", config);

    // Test tokenizer loading (this might take time)
    console.log("Testing tokenizer loading...");
    const { AutoTokenizer } = require("@xenova/transformers");

    const tokenizer = await AutoTokenizer.from_pretrained("Xenova/gpt2");
    console.log("Tokenizer loaded successfully");

    // Test BCO trainer with just a few examples
    const smallTrainData = trainData.slice(0, 4); // Just 4 examples for quick testing

    const trainer = new BCOTrainer({
      model: null, // No model needed for this test
      args: config,
      train_dataset: smallTrainData,
      eval_dataset: null,
      processing_class: tokenizer
    });

    console.log("BCO trainer created, testing loss computation...");

    // Test a single batch
    const batch = smallTrainData.slice(0, 2);
    const paddedBatch = trainer.data_collator.__call__(batch);
    const input = trainer._prepareBatchForTraining(paddedBatch);

    const { loss } = await trainer._computeLoss(input);
    console.log("Loss computation test - Loss:", loss);

    if (isNaN(loss) || !isFinite(loss)) {
      console.log("WARNING: Loss is NaN or infinite!");
    } else {
      console.log("SUCCESS: Loss computation working!");
    }

  } catch (error) {
    console.error("Test failed:", error.message);
    console.error(error.stack);
  }
}

testBCO();
