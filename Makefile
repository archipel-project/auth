.PHONY: migrate-run
migrate-run:
    sea-orm-cli migrate up -d ./migration/

.PHONY: migrate-rollback
migrate-rollback:
    sea-orm-cli migrate down -d ./migration/

.PHONY: migrate-reset
migrate-reset:
    sea-orm-cli migrate reset -d ./migration/

.PHONY: migrate-refresh
migrate-refresh:
    sea-orm-cli migrate refresh -d ./migration/

.PHONY: generate-entities
generate-entities:
    sea-orm-cli generate entity -o ./entities \
        --with-serde both --serde-skip-hidden-column --serde-skip-deserializing-primary-key
