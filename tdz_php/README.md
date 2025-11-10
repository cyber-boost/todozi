# Todozi PHP Bindings

PHP bindings for Todozi using ext-php-rs.

## Setup

1. Add to `Cargo.toml`:
```toml
[dependencies]
ext-php-rs = "0.13"
```

2. Build the extension:
```bash
cargo build --release
```

3. Copy `todozi.ini` to your PHP configuration directory:
```bash
cp todozi.ini /etc/php/8.x/mods-available/
```

4. Enable the extension:
```bash
php8enmod todozi
```

## Usage

```php
<?php
$todozi = new \Todozi\Todozi();
$taskId = $todozi->task("Complete project");
$todozi->done($taskId);
```

## Generated Files

- `todozi_php.rs` - Rust extension code (ext-php-rs)
- `Todozi.php` - PHP wrapper class
- `todozi.ini` - PHP extension configuration
- `composer.json` - Composer package definition
- `Cargo.toml.snippet` - Cargo configuration snippet
