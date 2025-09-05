from pydantic_settings import BaseSettings
from typing import List


class Settings(BaseSettings):
    PROJECT_NAME: str = "Rllm API"
    DESCRIPTION: str = "REST API for llm"
    DEBUG: bool = False
    VERSION: str = "1.0.0"
    API_V1_STR: str = "/api/v1"
    LOGGER_LEVEL: str = "DEBUG"

    ALLOWED_ORIGINS: List[str] = [
        "http://localhost",
        "http://localhost:8080",
        "http://localhost:3000"
    ]

    LOG_LEVEL: str = "DEBUG"

settings = Settings()
