use translator::Translator;

fn main() {
    println!("=== NLLB-200 Translation Test ===\n");

    let t = Translator::new();
    println!("Model ready: {}\n", t.is_model_ready());

    let tests = vec![
        ("ko", "en", "안녕하세요"),
        ("ko", "en", "한국에 오신 것을 환영합니다"),
        ("ko", "zh", "감사합니다"),
        ("ko", "ja", "좋은 아침입니다"),
    ];

    for (src, tgt, text) in &tests {
        match t.translate(text, src, tgt) {
            Ok(result) => println!("PASS [{}->{}] '{}' => '{}'", src, tgt, text, result),
            Err(e) => println!("FAIL [{}->{}] '{}' => ERROR: {}", src, tgt, text, e),
        }
    }

    println!("\n=== Test Complete ===");
}
