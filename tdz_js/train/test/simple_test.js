// Simple test of BCO loss computation
const { logSigmoid, sigmoid } = require('../bco');

// Test the log sigmoid function
console.log("Testing logSigmoid function:");
console.log("logSigmoid(0):", logSigmoid(0));
console.log("logSigmoid(1):", logSigmoid(1));
console.log("logSigmoid(-1):", logSigmoid(-1));
console.log("logSigmoid(10):", logSigmoid(10));
console.log("logSigmoid(-10):", logSigmoid(-10));

// Test edge cases
console.log("\nTesting edge cases:");
console.log("logSigmoid(500):", logSigmoid(500));
console.log("logSigmoid(-500):", logSigmoid(-500));

// Test BCO loss computation manually
console.log("\nTesting BCO loss computation:");

const beta = 0.1;
const delta = 0.0; // Initial delta

// Simulate some rewards (positive and negative)
const chosenRewards = [0.1, 0.2, -0.1];
const rejectedRewards = [-0.2, -0.3, 0.05];

console.log("Chosen rewards:", chosenRewards);
console.log("Rejected rewards:", rejectedRewards);

// Compute losses
const chosenLosses = chosenRewards.map(r => -logSigmoid(r - delta));
const rejectedLosses = rejectedRewards.map(r => -logSigmoid(-(r - delta)));

console.log("Chosen losses:", chosenLosses);
console.log("Rejected losses:", rejectedLosses);

// Check for NaN
const allLosses = [...chosenLosses, ...rejectedLosses];
const hasNaN = allLosses.some(l => isNaN(l) || !isFinite(l));

console.log("All losses:", allLosses);
console.log("Has NaN or infinite:", hasNaN);

if (!hasNaN) {
    const avgLoss = allLosses.reduce((a, b) => a + b, 0) / allLosses.length;
    console.log("Average loss:", avgLoss);
    console.log("SUCCESS: No NaN values!");
} else {
    console.log("FAILURE: Contains NaN or infinite values");
}
