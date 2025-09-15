#!/bin/bash

echo "🚀 Running MegaStore Search System Benchmarks"
echo "=============================================="
echo ""

# Create results directory
mkdir -p benchmark_results

echo "📊 Running Indexing Benchmarks..."
cargo bench indexing_benchmark 2>&1 | tee benchmark_results/indexing_results.txt

echo ""
echo "📊 Running Graph Benchmarks..."
cargo bench graph_benchmark 2>&1 | tee benchmark_results/graph_results.txt

echo ""
echo "📊 Running Search System Benchmarks..."
cargo bench search_benchmark 2>&1 | tee benchmark_results/search_results.txt

echo ""
echo "✅ All benchmarks completed!"
echo "📁 Results saved in benchmark_results/ directory"
echo ""
echo "📈 Quick Summary:"
echo "- Indexing performance: See benchmark_results/indexing_results.txt"
echo "- Graph performance: See benchmark_results/graph_results.txt"
echo "- Search performance: See benchmark_results/search_results.txt"
echo ""
echo "🎯 Key Metrics to Check:"
echo "- Product insertion rate (products/second)"
echo "- Search response time (microseconds)"
echo "- Recommendation generation time"
echo "- Scaling behavior with dataset size"