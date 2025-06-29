fn main() {
    vergen::EmitBuilder::builder()
        .all_git()
        .emit()
        .expect("Failed to emit vergen build instructions");
}

