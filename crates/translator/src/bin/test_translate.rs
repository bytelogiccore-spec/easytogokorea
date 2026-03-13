use translator::{Translator, EngineType};

fn main() {
    println!("=== Translation Integration Test ===\n");

    let t = Translator::new();
    println!("Current engine: {:?}", t.current_engine());
    println!("Available engines: {:?}\n", t.available_engines());

    let tests = vec![
        ("ko", "en", "안녕하세요"),
        ("ko", "en", "한국에 오신 것을 환영합니다"),
    ];

    // ─── Test Opus-MT ───
    println!("--- Opus-MT Tests ---");
    t.set_engine(EngineType::OpusMT);

    for (src, tgt, text) in &tests {
        match t.translate(text, src, tgt) {
            Ok(result) => println!("PASS [{}->{}] '{}' => '{}'", src, tgt, text, result),
            Err(e) => println!("FAIL [{}->{}] '{}' => ERROR: {}", src, tgt, text, e),
        }
    }

    // ─── Test NLLB-200 ───
    println!("\n--- NLLB-200 Tests ---");
    t.set_engine(EngineType::Nllb200);

    for (src, tgt, text) in &tests {
        match t.translate(text, src, tgt) {
            Ok(result) => println!("PASS [{}->{}] '{}' => '{}'", src, tgt, text, result),
            Err(e) => println!("FAIL [{}->{}] '{}' => ERROR: {}", src, tgt, text, e),
        }
    }

    println!("\n=== Test Complete ===");
}
