# rust-tokio-service-template

### How to use

1. Clone the repository

   ```sh
   git clone --depth 1 https://src.honbra.com/honbra/rust-tokio-service-template.git my-app
   ```

2. Go into the local copy

   ```sh
   cd my-app
   ```

3. Delete the `.git` folder (you don't need the commit history for a template)
    ```sh
    rm -rf .git
    ```

4. Initialize a new Git repository (optional)

   ```sh
   git init
   ```

5. Change the app name in `Cargo.toml` and `src/main.rs`

   ```diff
   Cargo.toml
   - name = "rust-tokio-service-template"
   + name = "my-app"
   ```

   ```diff
   src/main.rs
   - set_up_tracing("rust_tokio_service_template").context("Failed to set up tracing")?;
   + set_up_tracing("my_app").context("Failed to set up tracing")?;
   ```

6. Do your thing, write some code, commit cybercrime
    ```sh
    # I can't show you how to commit cybercrime, sorry
    ```

7. Run your app
   ```sh
   cargo run
   ```

### License

This template is licensed under the Unlicense. More information can be found in [the LICNSE file](./LICENSE).
