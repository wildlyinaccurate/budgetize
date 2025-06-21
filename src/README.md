# Budgetize

Categorize your spending so you can budget better!

## Local development

```
npx nodemon --watch src -e rs --exec cargo build
```

## Testing

```
./target/debug/budgetize --format westpac --input ./tests/sample_data/2025-04-westpac.csv
./target/debug/budgetize --format amex --input ./tests/sample_data/2025-05-amex-bare.csv
./target/debug/budgetize --format amex --input ./tests/sample_data/2025-05-amex-detailed.csv
```
