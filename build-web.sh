#!/bin/bash
set -e

echo "🎮 Building Crimsonland for Web..."

# Check for wasm target
if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
    echo "📦 Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Check for wasm-bindgen-cli
if ! command -v wasm-bindgen &> /dev/null; then
    echo "📦 Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli
fi

# Build for WASM (release for smaller size)
echo "🔨 Compiling to WebAssembly..."
cargo build --release --target wasm32-unknown-unknown

# Create dist directory
echo "📁 Setting up dist directory..."
rm -rf dist
mkdir -p dist

# Run wasm-bindgen to generate JS bindings
echo "🔗 Generating JavaScript bindings..."
wasm-bindgen --out-dir dist --target web \
    target/wasm32-unknown-unknown/release/crimsonland.wasm

# Copy web files and game assets
echo "📄 Copying web assets..."
cp web/index.html dist/
cp -r assets dist/ 2>/dev/null || true

# Optional: Optimize WASM with wasm-opt (if installed)
if command -v wasm-opt &> /dev/null; then
    echo "⚡ Optimizing WASM binary..."
    wasm-opt -Oz -o dist/crimsonland_bg.wasm dist/crimsonland_bg.wasm
else
    echo "💡 Tip: Install wasm-opt for smaller binary: cargo install wasm-opt"
fi

# Print size info
echo ""
echo "✅ Build complete!"
echo "📊 Output size:"
ls -lh dist/*.wasm
echo ""
echo "🚀 To test locally:"
echo "   cd dist && python3 -m http.server 8080"
echo "   Then open http://localhost:8080"
echo ""
echo "📤 To deploy to GitHub Pages:"
echo "   1. Push the dist/ folder to gh-pages branch, OR"
echo "   2. Configure GitHub Pages to serve from dist/ folder"
