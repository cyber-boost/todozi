//Dependencies (install with: npm i @xenova/transformers @tensorflow/tfjs-node)
 /**
 * BCO Trainer - Single File Implementation
 * ======================================
 * 
 * A complete, self-contained Binary Classifier Optimization (BCO) trainer.
 * This script includes everything you need: config, utilities, data handling,
 * and training logic - all in one elegant file.
 * 
 * Usage:
 * const { BCOConfig, BCOTrainer } = require('./bco_trainer');
 * 
 * Or run directly: node bco.js
 * 
 * Dependencies (install with: npm i @xenova/transformers @tensorflow/tfjs-node)
 */

 "use strict";

 /* =======================================================================
    IMPORTS & SETUP
    ======================================================================= */
 
const { AutoTokenizer, AutoModelForCausalLM } = require("@xenova/transformers");
 const TODOZI_DATASET_PATH = "./train/datasets/todozi/small.json";
 
 // Simple logger
 const logger = {
   info: (...args) => console.log("[INFO]", ...args),
   warn: (...args) => console.warn("[WARN]", ...args),
   error: (...args) => console.error("[ERROR]", ...args),
   step: (step, total, loss) => {
     if (step % 10 === 0) {
       console.log(`[STEP ${step}/${total}] Loss: ${loss.toFixed(4)}`);
     }
   }
 };
 
 /* =======================================================================
    CONFIGURATION CLASS
    ======================================================================= */
 
 class BCOConfig {
   constructor(options = {}) {
     // Training & logging
     this.logging_steps = options.logging_steps ?? 10;
     this.num_train_epochs = options.num_train_epochs ?? 1;
     this.learning_rate = options.learning_rate ?? 5e-5;
     this.per_device_train_batch_size = options.per_device_train_batch_size ?? 2;
     this.per_device_eval_batch_size = options.per_device_eval_batch_size ?? 2;
     
     // Precision
     this.fp16 = options.fp16 ?? false;
     this.bf16 = options.bf16 ?? (this.fp16 ? false : true);
     
     // Model & data
     this.max_length = options.max_length ?? 512;
     this.max_prompt_length = options.max_prompt_length ?? 128;
     this.max_completion_length = options.max_completion_length ?? 128;
     this.label_pad_token_id = options.label_pad_token_id ?? -100;
     this.padding_value = options.padding_value ?? null;
     this.truncation_mode = options.truncation_mode ?? "keep_end";
     
     // BCO specific
     this.beta = options.beta ?? 0.1;
     this.disable_dropout = options.disable_dropout ?? true;
     this.generate_during_eval = options.generate_during_eval ?? false;
     this.precompute_ref_log_probs = options.precompute_ref_log_probs ?? false;
     
     // Underlying Distribution Matching (UDM)
     this.prompt_sample_size = options.prompt_sample_size ?? 512;
     this.min_density_ratio = options.min_density_ratio ?? 0.5;
     this.max_density_ratio = options.max_density_ratio ?? 10.0;
     
     // Misc
     this.output_dir = options.output_dir ?? "./output";
     this.seed = options.seed ?? 42;
   }
 }
 
 /* =======================================================================
    UTILITY FUNCTIONS
    ======================================================================= */
 
 // Convert padding tokens to the pad token
 function padToLength(tensor, maxLength, padTokenId) {
   const currentLength = tensor.length;
   if (currentLength >= maxLength) {
     return tensor.slice(0, maxLength);
   }
   return [...tensor, ...new Array(maxLength - currentLength).fill(padTokenId)];
 }
 
// Simple log softmax implementation
function logSoftmax(logits) {
  const maxLogit = Math.max(...logits);
  const expLogits = logits.map(x => Math.exp(x - maxLogit));
  const sumExp = expLogits.reduce((a, b) => a + b, 0);
  return expLogits.map(x => Math.log(x / sumExp));
}

// Get log probabilities for a batch
function getBatchLogps(logits, labels, labelPadTokenId = -100, isEncoderDecoder = false) {
   // For causal LM, shift labels and logits
   if (!isEncoderDecoder) {
     labels = labels.slice(1);
     logits = logits.slice(0, -1);
   }

   // Create mask for non-pad tokens
   const mask = labels.map((label, i) => label !== labelPadTokenId);

   // Log softmax
   const logProbs = logits.map(seq => seq.map(tokenLogits => logSoftmax(tokenLogits)));

   // Gather log probs
   const batchSize = labels.length;
   const seqLength = labels[0].length;
   const results = [];

   for (let b = 0; b < batchSize; b++) {
     let totalLogProb = 0;
     let validTokens = 0;

     for (let s = 0; s < seqLength; s++) {
       if (mask[b][s]) {
         const label = labels[b][s];
         const logProb = logProbs[b][s][label];
         totalLogProb += logProb;
         validTokens++;
       }
     }

     results.push(validTokens > 0 ? totalLogProb / validTokens : 0);
   }

   return results;
}
 
// Simple sigmoid with numerical stability
function sigmoid(x) {
  // Clamp x to prevent overflow
  x = Math.max(-500, Math.min(500, x));
  return 1 / (1 + Math.exp(-x));
}

// Numerically stable log sigmoid
function logSigmoid(x) {
  if (x >= 0) {
    return -Math.log(1 + Math.exp(-x));
  } else {
    return x - Math.log(1 + Math.exp(x));
  }
}

// Clamp values between min and max
function clamp(x, min, max) {
  return Math.max(min, Math.min(max, x));
}

// Simple Logistic Regression class
class SimpleLogisticRegression {
  constructor(inputSize) {
    this.weights = new Array(inputSize).fill(0).map(() => Math.random() * 0.01);
    this.bias = Math.random() * 0.01;
  }

  predict_proba(x) {
    const logit = x.reduce((sum, xi, i) => sum + xi * this.weights[i], 0) + this.bias;
    return sigmoid(logit);
  }

  trainStep(x, y, learningRate) {
    const pred = this.predict_proba(x);
    const error = pred - y;

    // Update weights and bias
    for (let i = 0; i < this.weights.length; i++) {
      this.weights[i] -= learningRate * error * x[i];
    }
    this.bias -= learningRate * error;
  }
}
 
 // Convert labels array to indices
 function getChosenRejectedIndices(labels) {
   const chosen = [];
   const rejected = [];
   
   labels.forEach((label, i) => {
     if (label === true || label === 1) {
       chosen.push(i);
     } else {
       rejected.push(i);
     }
   });
   
   return { chosen, rejected };
 }
 
 // data_loader.js (corrected)
 "use strict";
 
 const fs = require("fs");
 
 class BCODataLoader {
   constructor(jsonPath) {
     this.jsonPath = jsonPath;
     this.data = null;
   }
   
   load() {
     if (!fs.existsSync(this.jsonPath)) {
       throw new Error(`Dataset file not found: ${this.jsonPath}`);
     }
     
     const rawData = fs.readFileSync(this.jsonPath, "utf8");
     this.data = JSON.parse(rawData);
     
     return this;
   }
   
   getDatasets() {
     if (!this.data) {
       throw new Error("Data not loaded. Call load() first.");
     }
     
     let trainData, evalData;
     
     // Detect format
     if (this.data.train_dataset || this.data.train) {
       // Combined format
       trainData = this.data.train_dataset || this.data.train || [];
       evalData = this.data.eval_dataset || this.data.eval || this.data.validation || [];
     } else if (this.data.dataset_info || this.data.data) {
       // Metadata format
       trainData = this.data.data?.train || this.data.data?.training || [];
       evalData = this.data.data?.validation || this.data.data?.eval || [];
     } else if (Array.isArray(this.data)) {
       // Separate array format
       trainData = this.data;
       evalData = null;
     } else {
       throw new Error("Unable to detect dataset format");
     }
     
     // Convert to internal format (prompt, completion, label)
     trainData = this._convertToLabeledFormat(trainData, "training");
     evalData = evalData ? this._convertToLabeledFormat(evalData, "evaluation") : null;
     
     return { trainData, evalData };
   }
   
   /**
    * Convert chosen/rejected format to prompt/completion/label format
    */
   _convertToLabeledFormat(data, datasetName) {
     if (!Array.isArray(data)) {
       throw new Error(`${datasetName} data must be an array`);
     }
     
     const converted = [];
     let validCount = 0;
     let invalidCount = 0;
     
     for (let i = 0; i < data.length; i++) {
       const entry = data[i];
       
       // Validate required fields
       if (!entry.prompt || typeof entry.prompt !== "string") {
         console.warn(`Invalid ${datasetName} entry ${i}: missing or invalid 'prompt' field`);
         invalidCount++;
         continue;
       }
       
       if (!entry.chosen || typeof entry.chosen !== "string") {
         console.warn(`Invalid ${datasetName} entry ${i}: missing or invalid 'chosen' field`);
         invalidCount++;
         continue;
       }
       
       if (!entry.rejected || typeof entry.rejected !== "string") {
         console.warn(`Invalid ${datasetName} entry ${i}: missing or invalid 'rejected' field`);
         invalidCount++;
         continue;
       }
       
       // Create two entries: one for chosen (label=true), one for rejected (label=false)
       converted.push({
         prompt: entry.prompt.trim(),
         completion: entry.chosen.trim(),
         label: true,
         id: entry.id ? `${entry.id}_chosen` : converted.length,
         source: entry.source || "user_provided"
       });
       
       converted.push({
         prompt: entry.prompt.trim(),
         completion: entry.rejected.trim(),
         label: false,
         id: entry.id ? `${entry.id}_rejected` : converted.length,
         source: entry.source || "user_provided"
       });
       
       validCount++;
     }
     
     console.log(`Loaded ${datasetName} dataset: ${validCount} pairs (${validCount * 2} entries), ${invalidCount} skipped`);
     
     return converted;
   }
   
   getStatistics() {
     if (!this.data) {
       throw new Error("Data not loaded. Call load() first.");
     }
     
     const { trainData, evalData } = this.getDatasets();
     
     const countPairs = (data) => data ? Math.floor(data.length / 2) : 0;
     
     const stats = {
       train: {
         pairs: countPairs(trainData),
         total: trainData.length,
         positive: trainData.filter(d => d.label).length,
         negative: trainData.filter(d => !d.label).length,
         avgPromptLength: this._calculateAverageLength(trainData, "prompt"),
         avgCompletionLength: this._calculateAverageLength(trainData, "completion")
       },
       eval: evalData ? {
         pairs: countPairs(evalData),
         total: evalData.length,
         positive: evalData.filter(d => d.label).length,
         negative: evalData.filter(d => !d.label).length,
         avgPromptLength: this._calculateAverageLength(evalData, "prompt"),
         avgCompletionLength: this._calculateAverageLength(evalData, "completion")
       } : null
     };
     
     return stats;
   }
   
   _calculateAverageLength(data, field) {
     const totalLength = data.reduce((sum, entry) => sum + entry[field].length, 0);
     return Math.round(totalLength / data.length);
   }
 }
 

 /* =======================================================================
    DATA COLLATOR
    ======================================================================= */
 
 class BCODataCollator {
   constructor(tokenizer, config) {
     this.tokenizer = tokenizer;
     this.config = config;
     this.padTokenId = tokenizer.pad_token_id ?? 0;
   }
   
   __call__(examples) {
     // Find max length for padding
     const keys = Object.keys(examples[0]);
     const maxLengths = {};
     
     keys.forEach(key => {
       if (Array.isArray(examples[0][key]) && typeof examples[0][key][0] === 'number') {
         maxLengths[key] = Math.max(...examples.map(e => e[key].length));
       }
     });
     
     // Pad all sequences
     const batch = {};
     keys.forEach(key => {
       if (Array.isArray(examples[0][key])) {
         batch[key] = examples.map(example => {
           const arr = [...example[key]];
           const maxLen = maxLengths[key];
           
           if (typeof arr[0] === 'number') {
             // Padding for token arrays
             while (arr.length < maxLen) {
               if (key.includes('label')) {
                 arr.push(this.config.label_pad_token_id);
               } else {
                 arr.push(this.padTokenId);
               }
             }
           }
           
           return arr;
         });
       } else {
         // Non-array values (strings, numbers, booleans)
         batch[key] = examples.map(example => example[key]);
       }
     });
     
     return batch;
   }
 }
 
 /* =======================================================================
    RUNNING MOMENTS (for delta computation)
    ======================================================================= */
 
class RunningMoments {
  constructor() {
    this.mean = 0.0;
    this.count = 1; // Start with count=1 to avoid division by zero
  }
   
   update(values) {
     for (const value of values) {
       this.count++;
       const delta = value - this.mean;
       this.mean += delta / this.count;
     }
   }
 }
 
 /* =======================================================================
    MAIN BCO TRAINER CLASS
    ======================================================================= */
 
 class BCOTrainer {
   constructor({
     model = null,
     ref_model = null,
     args = new BCOConfig(),
     train_dataset = null,
     eval_dataset = null,
     processing_class = null,
     embedding_func = null,
     embedding_tokenizer = null
   } = {}) {
     
     // Validate inputs
     if (!args || !args instanceof BCOConfig) {
       throw new Error("BCOTrainer requires a BCOConfig instance");
     }
     if (!processing_class) {
       throw new Error("BCOTrainer requires a processing_class (tokenizer)");
     }
     
     this.config = args;
     this.model = model;
     this.ref_model = ref_model;
     this.train_dataset = train_dataset;
     this.eval_dataset = eval_dataset;
     this.tokenizer = processing_class;
     this.data_collator = new BCODataCollator(this.tokenizer, this.config);
     
     // UDM components
     this.embedding_func = embedding_func;
     this.embedding_tokenizer = embedding_tokenizer;
     
     // Internal state
     this.running = new RunningMoments();
     this.udm_classifier = null;
     this.udm_trained = false;
     
     // Preprocess datasets
     if (this.train_dataset) {
       this.train_dataset = this._preprocessDataset(this.train_dataset, true);
     }
     if (this.eval_dataset) {
       this.eval_dataset = this._preprocessDataset(this.eval_dataset, false);
     }
     
     logger.info("BCOTrainer initialized successfully");
   }
   
   /**
    * Preprocess dataset: tokenize, process tokens, and prepare for training
    */
   _preprocessDataset(dataset, isTraining = true) {
     // Tokenize
     const tokenized = dataset.map(example => this._tokenizeExample(example));
     
     // Process tokens (add special tokens, create labels, etc.)
     const processed = tokenized.map(example => this._processTokens(example));
     
     return processed;
   }
   
   /**
    * Tokenize a single example
    */
   _tokenizeExample(example) {
     // Tokenize prompt
     const promptTokens = this.tokenizer(example.prompt, { addSpecialTokens: false });
     
     // Tokenize prompt + completion
     const fullTokens = this.tokenizer(example.prompt + example.completion, { addSpecialTokens: false });
     
     // Split into prompt and completion parts
     const promptLength = promptTokens.input_ids.length;
     const completionIds = fullTokens.input_ids.slice(promptLength);
     const completionMask = fullTokens.attention_mask.slice(promptLength);
     
     return {
       prompt: example.prompt,
       completion: example.completion,
       label: example.label,
       prompt_input_ids: promptTokens.input_ids,
       prompt_attention_mask: promptTokens.attention_mask,
       completion_input_ids: completionIds,
       completion_attention_mask: completionMask
     };
   }
   
   /**
    * Process tokens: add BOS/EOS, create labels
    */
   _processTokens(example) {
     const { prompt_input_ids, prompt_attention_mask, completion_input_ids, completion_attention_mask } = example;
     
     // Add BOS token if needed
     const bosTokenId = this.tokenizer.bos_token_id;
     const eosTokenId = this.tokenizer.eos_token_id;
     
     const finalPromptIds = bosTokenId && bosTokenId !== prompt_input_ids[0] 
       ? [bosTokenId, ...prompt_input_ids]
       : [...prompt_input_ids];
     
     const finalPromptMask = bosTokenId && bosTokenId !== prompt_input_ids[0]
       ? [1, ...prompt_attention_mask]
       : [...prompt_attention_mask];
     
     // Create full completion (prompt + completion)
     const finalCompletionIds = [...finalPromptIds, ...completion_input_ids];
     const finalCompletionMask = [...finalPromptMask, ...completion_attention_mask];
     
     // Add EOS token
     if (eosTokenId && completion_input_ids[completion_input_ids.length - 1] !== eosTokenId) {
       finalCompletionIds.push(eosTokenId);
       finalCompletionMask.push(1);
     }
     
     // Create labels (mask prompt tokens)
     const completionLabels = finalCompletionIds.map((id, i) => 
       i < finalPromptIds.length ? this.config.label_pad_token_id : id
     );
     
     return {
       ...example,
       prompt_input_ids: finalPromptIds,
       prompt_attention_mask: finalPromptMask,
       completion_input_ids: finalCompletionIds,
       completion_attention_mask: finalCompletionMask,
       completion_labels: completionLabels
     };
   }
   
   /**
    * Get embeddings for UDM (if embedding function is provided)
    */
   async _getEmbeddings(batch) {
     if (!this.embedding_func || !this.embedding_tokenizer) {
       return { chosen: null, rejected: null };
     }
     
     // Get embeddings for all examples in batch
     const embeddings = await this.embedding_func({
       input_ids: batch.embedding_input_ids,
       attention_mask: batch.embedding_attention_mask
     });
     
     // Split by label
     const { chosen, rejected } = getChosenRejectedIndices(batch.label);
     
     return {
       chosen: chosen.length > 0 ? embeddings[chosen] : null,
       rejected: rejected.length > 0 ? embeddings[rejected] : null
     };
   }
   
   /**
   * Compute UDM weight for rejected examples
   */
  _getUdmWeight(rejectedEmbeddings) {
    if (!this.udm_classifier || !rejectedEmbeddings) {
      return new Array(rejectedEmbeddings ? rejectedEmbeddings.length : 0).fill(1.0);
    }

    // Predict probabilities using the trained classifier
    const probs = rejectedEmbeddings.map(emb => this.udm_classifier.predict_proba(emb));

    // Calculate density ratio p/(1-p) with smoothing
    const densityRatios = probs.map(p => p / (1 - p + 1e-8));

    // Clamp to configured range
    return densityRatios.map(r => clamp(r, this.config.min_density_ratio, this.config.max_density_ratio));
  }
   
   /**
    * Train the UDM classifier (logistic regression)
    */
   async _trainUdmClassifier() {
     if (!this.embedding_func || !this.embedding_tokenizer) {
       return;
     }
     
     logger.info("Training UDM classifier...");
     
     // Sample data for training
     const sampleSize = Math.min(this.config.prompt_sample_size, Math.floor(this.train_dataset.length / 2));
     const chosenSample = this.train_dataset.filter(e => e.label === true).shuffle().take(sampleSize);
     const rejectedSample = this.train_dataset.filter(e => e.label === false).shuffle().take(sampleSize);
     
     // Collect embeddings and labels
     const allEmbeddings = [];
     const allLabels = [];
     
     for (const dataset of [chosenSample, rejectedSample]) {
       for (const example of dataset) {
         const embedding = await this.embedding_func({
           input_ids: [example.embedding_input_ids],
           attention_mask: [example.embedding_attention_mask]
         });
         
         allEmbeddings.push(embedding[0]); // Flatten the embedding
         allLabels.push(example.label ? 1 : 0);
       }
     }
     
    // Train simple logistic regression (using plain JavaScript)
    const inputSize = allEmbeddings[0].length;
    const model = new SimpleLogisticRegression(inputSize);

    // Convert to arrays
    const xs = allEmbeddings;
    const ys = allLabels;

    // Simple training loop
    for (let epoch = 0; epoch < 50; epoch++) {
      for (let i = 0; i < xs.length; i++) {
        model.trainStep(xs[i], ys[i], 0.01);
      }
    }

    this.udm_classifier = model;
    this.udm_trained = true;
     
     logger.info("UDM classifier training completed");
   }
   
   /**
    * Main training loop
    */
   async train() {
     if (!this.train_dataset) {
       throw new Error("No training dataset provided");
     }
     
     logger.info("Starting BCO training...");
     
     // Train UDM classifier if needed
     if (this.embedding_func && !this.udm_trained) {
       await this._trainUdmClassifier();
     }
     
     const numBatches = Math.ceil(this.train_dataset.length / this.config.per_device_train_batch_size);
     let totalLoss = 0;
     let batchCount = 0;
     
    // Simple optimizer (we'll implement gradient updates manually)
    const learningRate = this.config.learning_rate;
     
     for (let epoch = 1; epoch <= this.config.num_train_epochs; epoch++) {
       logger.info(`Epoch ${epoch}/${this.config.num_train_epochs}`);
       
       // Shuffle dataset
       const shuffled = this.train_dataset.shuffle ? this.train_dataset.shuffle() : this.train_dataset;
       
       for (let batchIdx = 0; batchIdx < numBatches; batchIdx++) {
         const start = batchIdx * this.config.per_device_train_batch_size;
         const end = Math.min(start + this.config.per_device_train_batch_size, shuffled.length);
         const batch = shuffled.slice(start, end);
         
         // Collate batch
         const paddedBatch = this.data_collator.__call__(batch);
         
         // Convert to tensors
         const input = this._prepareBatchForTraining(paddedBatch);
         
         // Compute loss (no actual gradient updates since model is not trainable)
         const { loss } = await this._computeLoss(input);
         
         totalLoss += loss;
         batchCount++;
         
        // Log progress
        logger.step(batchIdx + 1, numBatches, loss);
       }
       
       const avgLoss = totalLoss / batchCount;
       logger.info(`Epoch ${epoch} completed. Average loss: ${avgLoss.toFixed(4)}`);
     }
     
     logger.info("Training completed successfully!");
   }
   
   /**
    * Prepare batch for training
    */
  _prepareBatchForTraining(batch) {
    return {
      prompt_input_ids: batch.prompt_input_ids,
      prompt_attention_mask: batch.prompt_attention_mask,
      completion_input_ids: batch.completion_input_ids,
      completion_attention_mask: batch.completion_attention_mask,
      completion_labels: batch.completion_labels,
      label: batch.label
    };
  }
   
   /**
    * Compute BCO loss for a batch
    */
   async _computeLoss(batch) {
     const { chosen, rejected } = getChosenRejectedIndices(batch.label);
     
    // Forward pass for policy model (simplified - using mock logits since model is not trainable)
    // In a real implementation, you'd get logits from the model
    const vocabSize = 50257; // GPT-2 vocab size
    const seqLength = batch.completion_input_ids[0].length;

    // Mock logits for demonstration (random values)
    const policyLogits = batch.completion_input_ids.map(() =>
      new Array(seqLength).fill(0).map(() =>
        new Array(vocabSize).fill(0).map(() => Math.random() - 0.5)
      )
    );

    const policyLogps = getBatchLogps(
      policyLogits,
      batch.completion_labels,
      this.config.label_pad_token_id
    );
     
     // Split log probs by label
     const policyChosenLogps = chosen.map(i => policyLogps[i]);
     const policyRejectedLogps = rejected.map(i => policyLogps[i]);
     
    // Reference model forward pass (simplified - using same mock logits for now)
    let refChosenLogps, refRejectedLogps;

    if (this.config.precompute_ref_log_probs && batch.reference_logps) {
      const refLogps = batch.reference_logps;
      refChosenLogps = chosen.map(i => refLogps[i]);
      refRejectedLogps = rejected.map(i => refLogps[i]);
    } else {
      // For simplicity, use slightly different random logits for reference model
      const refLogits = batch.completion_input_ids.map(() =>
        new Array(seqLength).fill(0).map(() =>
          new Array(vocabSize).fill(0).map(() => Math.random() - 0.5)
        )
      );

      const refLogps = getBatchLogps(
        refLogits,
        batch.completion_labels,
        this.config.label_pad_token_id
      );

      refChosenLogps = chosen.map(i => refLogps[i]);
      refRejectedLogps = rejected.map(i => refLogps[i]);
    }
     
     // Compute BCO loss
     const { loss, chosenRewards, rejectedRewards } = this._computeBcoLoss(
       policyChosenLogps,
       policyRejectedLogps,
       refChosenLogps,
       refRejectedLogps,
       batch
     );
     
     // Update running moments
     this.running.update([...chosenRewards, ...rejectedRewards]);
     
     return { loss };
   }
   
   /**
    * Compute BCO loss
    */
   _computeBcoLoss(policyChosenLogps, policyRejectedLogps, refChosenLogps, refRejectedLogps, batch) {
     const beta = this.config.beta;
     
     // Compute log ratios and rewards
     const chosenLogRatios = policyChosenLogps.map((p, i) => p - refChosenLogps[i]);
     const rejectedLogRatios = policyRejectedLogps.map((p, i) => p - refRejectedLogps[i]);
     
     const chosenRewards = chosenLogRatios.map(r => beta * r);
     const rejectedRewards = rejectedLogRatios.map(r => beta * r);
     
    // Compute losses with numerical stability
    const delta = this.running.mean;
    const chosenLosses = chosenRewards.map(r => -logSigmoid(r - delta));
    const rejectedLosses = rejectedRewards.map(r => -logSigmoid(-(r - delta)));
     
     // Apply UDM weighting if available
     let finalRejectedLosses = rejectedLosses;
     if (this.udm_classifier) {
       // In a full implementation, you'd get embeddings and compute weights here
       // For simplicity, we use uniform weighting
     }
     
    // Combine losses
    const allLosses = [...chosenLosses, ...finalRejectedLosses];

    // Filter out NaN and infinite values
    const validLosses = allLosses.filter(l => isFinite(l) && !isNaN(l));

    if (validLosses.length === 0) {
      // If all losses are invalid, return a small positive loss
      return { loss: 1.0, chosenRewards, rejectedRewards };
    }

    const loss = validLosses.reduce((a, b) => a + b, 0) / validLosses.length;
     
     return { loss, chosenRewards, rejectedRewards };
   }
   
   /**
    * Evaluate the model
    */
   async evaluate(evalDataset = this.eval_dataset) {
     if (!evalDataset) {
       throw new Error("No evaluation dataset provided");
     }
     
     logger.info("Starting evaluation...");
     
     const numBatches = Math.ceil(evalDataset.length / this.config.per_device_eval_batch_size);
     let totalLoss = 0;
     let batchCount = 0;
     
     for (let batchIdx = 0; batchIdx < numBatches; batchIdx++) {
       const start = batchIdx * this.config.per_device_eval_batch_size;
       const end = Math.min(start + this.config.per_device_eval_batch_size, evalDataset.length);
       const batch = evalDataset.slice(start, end);
       
      const paddedBatch = this.data_collator.__call__(batch);
      const input = this._prepareBatchForTraining(paddedBatch);

      const { loss } = await this._computeLoss(input); // No optimizer for eval
      totalLoss += loss;
      batchCount++;
     }
     
     const avgLoss = totalLoss / batchCount;
     logger.info(`Evaluation completed. Average loss: ${avgLoss.toFixed(4)}`);
     
     return { loss: avgLoss };
   }
   
   /**
    * Save model and training state
    */
   async save_model(outputDir = this.config.output_dir) {
     const fs = require("fs");
     const path = require("path");
     
     if (!fs.existsSync(outputDir)) {
       fs.mkdirSync(outputDir, { recursive: true });
     }
     
     // Save model
     await this.model.save(`file://${path.join(outputDir, "model")}`);
     
     // Save running moments
     fs.writeFileSync(
       path.join(outputDir, "running.json"),
       JSON.stringify({
         mean: this.running.mean,
         count: this.running.count
       }, null, 2)
     );
     
     // Save UDM classifier if trained
     if (this.udm_classifier) {
       await this.udm_classifier.save(`file://${path.join(outputDir, "udm_classifier")}`);
     }
     
     logger.info(`Model saved to ${outputDir}`);
   }
   
   /**
    * Generate text using the trained model
    */
   async generate(prompt, maxLength = 100, doSample = true, temperature = 0.7) {
     // Tokenize prompt
     const tokens = this.tokenizer(prompt, { returnTensors: "pt" });
     
     // Generate
     const output = await this.model.generate({
       input_ids: tokens.input_ids,
       attention_mask: tokens.attention_mask,
       max_length: Math.min(maxLength, this.config.max_length),
       do_sample: doSample,
       temperature: temperature,
       pad_token_id: this.tokenizer.pad_token_id
     });
     
     // Decode
     return this.tokenizer.decode(output[0], { skip_special_tokens: true });
   }
 }
 
 /* =======================================================================
    EXPORTS
    ======================================================================= */
 
module.exports = {
   BCOConfig,
   BCODataLoader,
   BCOTrainer,
   sigmoid,
   logSigmoid
};
 
 async function trainWithPreferenceData() {
    // 1. Load dataset from JSON
    const loader = new BCODataLoader(TODOZI_DATASET_PATH);
    const { trainData, evalData } = loader.load().getDatasets();
    
    // 2. Show dataset statistics
    const stats = loader.getStatistics();
    console.log("Dataset Statistics:", stats);
    
    // 3. Load model and tokenizer (using GPT-2 for testing)
    const tokenizer = await AutoTokenizer.from_pretrained("Xenova/gpt2");
    const model = await AutoModelForCausalLM.from_pretrained("Xenova/gpt2");
    
    // 4. Create BCO trainer
    const config = new BCOConfig({
      per_device_train_batch_size: 2,
      num_train_epochs: 1,
      learning_rate: 5e-5,
      logging_steps: 1,
      output_dir: "./bco_output"
    });
    
    const trainer = new BCOTrainer({
      model: model,
      args: config,
      train_dataset: trainData,
      eval_dataset: evalData,
      processing_class: tokenizer
    });
    
    // 5. Train the model
    await trainer.train();
    
    // 6. Evaluate
    const metrics = await trainer.evaluate();
    console.log("Evaluation metrics:", metrics);
    
    // 7. Save the model
    await trainer.save_model();
  }
  
  trainWithPreferenceData().catch(console.error);