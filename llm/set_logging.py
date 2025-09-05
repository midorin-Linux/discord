import logging

def setup_logging(logger_level: str):
    log_level_str = logger_level.upper()
    try:
        log_level = getattr(logging, log_level_str)
    except AttributeError:
        log_level = logging.INFO
        print(f"Warning: Invalid LOGGER_LEVEL '{log_level_str}' specified. Falling back to INFO level.")

    dt_fmt = "%Y-%m-%d %H:%M:%S"
    formatter = logging.Formatter("[{asctime}] [{levelname:<8}] {name}: {message}", dt_fmt, style="{")

    console_handler = logging.StreamHandler()
    console_handler.setFormatter(formatter)

    root_logger = logging.getLogger()
    root_logger.setLevel(log_level)

    if root_logger.handlers:
        for handler in root_logger.handlers[:]:
            root_logger.removeHandler(handler)
    root_logger.addHandler(console_handler)

    logger = logging.getLogger(__name__)
    logger.info(f"Program started with {logging.getLevelName(log_level)} level logging.")