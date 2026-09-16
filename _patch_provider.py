import sys
sys.stdout.reconfigure(encoding='utf-8')

with open(r'crates/config/src/lib.rs', 'r', encoding='utf-8') as f:
    text = f.read()

# Add antigravity_model and antigravity_base_url to the load() initializer
# Find xai_model in load()
xai_model_block = '''            xai_model: std::env::var("XAI_MODEL")
                .or_else(|_| std::env::var("GROK_MODEL"))
                .ok()
                .filter(|v| !v.trim().is_empty()),'''

antigravity_model_block = '''            xai_model: std::env::var("XAI_MODEL")
                .or_else(|_| std::env::var("GROK_MODEL"))
                .ok()
                .filter(|v| !v.trim().is_empty()),
            antigravity_model: std::env::var("ANTIGRAVITY_MODEL")
                .or_else(|_| std::env::var("GOOGLE_ANTIGRAVITY_MODEL"))
                .ok()
                .filter(|v| !v.trim().is_empty()),'''

text = text.replace(xai_model_block, antigravity_model_block)

# Find xai_base_url in load()
xai_url_block = '''            xai_base_url: std::env::var("XAI_BASE_URL")
                .or_else(|_| std::env::var("GROK_BASE_URL"))
                .ok()
                .filter(|v| !v.trim().is_empty()),'''

antigravity_url_block = '''            xai_base_url: std::env::var("XAI_BASE_URL")
                .or_else(|_| std::env::var("GROK_BASE_URL"))
                .ok()
                .filter(|v| !v.trim().is_empty()),
            antigravity_base_url: std::env::var("ANTIGRAVITY_BASE_URL")
                .or_else(|_| std::env::var("GOOGLE_ANTIGRAVITY_BASE_URL"))
                .ok()
                .filter(|v| !v.trim().is_empty()),'''

text = text.replace(xai_url_block, antigravity_url_block)

with open(r'crates/config/src/lib.rs', 'w', encoding='utf-8') as f:
    f.write(text)
print('Done: added load() fields')
