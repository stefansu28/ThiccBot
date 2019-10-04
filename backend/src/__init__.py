from flask import Flask, request, abort, redirect, url_for, g, jsonify
from flask_sqlalchemy import SQLAlchemy
from flask_migrate import Migrate
from flask_restplus import Api
from flask_cors import CORS
import os
from .config import Config
from .constants import CONSTANTS


from flask_dance.contrib.discord import make_discord_blueprint, discord as dAuth

db = SQLAlchemy()
migrate = Migrate()
api = Api()
from src.routes import (
    aliasNs,
    discordNs,
    keyNs,
    quotesNs,
    albumsNs,
    lastFmNs,
    counterNs,
    standingsNs,
)

namespaces = [
    aliasNs,
    discordNs,
    keyNs,
    quotesNs,
    albumsNs,
    lastFmNs,
    counterNs,
    standingsNs,
]
isDev = os.getenv("FLASK_ENV", "PROD") == "development"
BOT_API_TOKEN = os.getenv("BOT_API_TOKEN", "BOT_API_TOKEN")
DISCORD_ID = os.getenv("DISCORD_CLIENT_ID", "DISCORD_CLIENT_ID")
DISCORD_SECRET = os.getenv("DISCORD_CLIENT_SECRET", "DISCORD_CLIENT_SECRET")


def create_app(config_class=Config):
    my_app = Flask(__name__)
    my_app.config.from_object(config_class)
    api.init_app(my_app)
    for ns in namespaces:
        api.add_namespace(ns)
    db.init_app(my_app)
    migrate.init_app(my_app, db)
    if "SECRET_KEY" in os.environ and not my_app.config["TESTING"]:
        my_app.secret_key = os.environ["SECRET_KEY"]
    else:
        my_app.logger.warning(
            "PLEASE SET A SECRET KEY, USING A DEFAULT KEY IS SAD TIMES"
        )
        my_app.secret_key = "supersekrit"
    # CORS(my_app)
    blueprint = make_discord_blueprint(
        client_id=DISCORD_ID,
        client_secret=DISCORD_SECRET,
        scope=["identify", "guilds"],
    )
    # CORS(blueprint)
    my_app.register_blueprint(blueprint, url_prefix="/login")

    return my_app


app = create_app(Config)
from src.routes.login import *
from src.models import (
    Alias,
    DiscordServer,
    ServerGroup,
    KeyWords,
    Quotes,
    Album,
    AlbumEntry,
    Counter,
)

# db.create_all(app=app)


@app.route("/api/health")
def healthRoute():
    return jsonify(CONSTANTS)


@app.before_request
def before_request():
    is_user = dAuth.authorized
    print(f"is_user: {is_user}")
    api_key_passed = request.headers.get("bot-token", "")
    allow_debug = app.config["TESTING"] or (
        isDev and request.headers.get("Host", "") == "localhost:5000"
    )
    g.is_bot = api_key_passed == BOT_API_TOKEN
    not_authorized = not (is_user or g.is_bot or allow_debug)
    print(f"not_authorized: {not_authorized}")
    in_allowed_route = request.endpoint not in (
        "login",
        "discord.login",
        "discord.authorized",
    )
    print(f"in_allowed_route: {in_allowed_route}")
    # if not_authorized and request.endpoint not in (
    #     "login",
    #     "discord.login",
    #     "discord.authorized",
    # ):
    #     return redirect(url_for("login"))
    # if not allow_debug and not_authorized and not in_allowed_route:
    #     return abort(403)
    g.is_bot = api_key_passed == BOT_API_TOKEN
