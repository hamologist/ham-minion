import os

import discord
from discord.ext import commands

TOKEN = os.environ['HAM_MINON_TOKEN']
GUILD_ID: str = os.environ['HAM_MINON_DEV_GUILD_ID']

intents = discord.Intents.default()
intents.message_content = True

class Client(commands.Bot):
    async def on_ready(self):
        print(f'Logged on as {self.user}')

        guild = discord.Object(id=GUILD_ID)
        synced = await self.tree.sync(guild=guild)
        print(f'Synced {len(synced)} commands to guild {guild.id}')


client = Client(command_prefix='>', intents=intents)
guild = discord.Object(id=GUILD_ID)

@client.tree.command(name="python-ping", guild=guild)
async def python_ping(interaction: discord.Interaction):
    await interaction.response.send_message('pong')

client.run(TOKEN)

