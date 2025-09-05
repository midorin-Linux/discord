import set_logging
from api.v1.api import api_router
from core.config import settings

import logging
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

logger = logging.getLogger(__name__)

class RllmAPI:
    def __init__(self):
        try:
            self.app = self._create_app()
            self._add_middleware()
            self._add_router()
            logger.info("Initializing app successful")
        except Exception as e:
            logger.error(f"Error creating app: {e}")

    def _create_app(self) -> FastAPI:
        app = FastAPI(
            title=settings.PROJECT_NAME,
            description=settings.DESCRIPTION,
            version=settings.VERSION,
            docs_url=f"/docs",
            redoc_url=f"/redoc",
            openapi_url=f"/openapi.json",
        )
        return app

    def _add_middleware(self):
        self.app.add_middleware(
            CORSMiddleware,
            allow_origins=settings.ALLOWED_ORIGINS,
            allow_credentials=True,
            allow_methods=["*"],
            allow_headers=["*"],
        )

    def _add_router(self):
        self.app.include_router(
            api_router,
            prefix=settings.API_V1_STR,
        )

set_logging.setup_logging(settings.LOG_LEVEL)
rllm_api = RllmAPI()

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(rllm_api.app, host="0.0.0.0", port=8000)
