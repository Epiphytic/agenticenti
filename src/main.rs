use agenticenti::generated::prompts;

fn main() {
    // Placeholder — will be replaced by clap CLI in next task
    for name in prompts::all_role_names() {
        println!("role: {}", name);
    }
}
