import discord
import requests																													

# API Key de Mistral
MISTRAL_API_KEY = "CJ3scWQpJqtggxqBb2wRqGrCwDUk3B5M"

# URL de la API de Mistral
url = "https://api.mistral.ai/v1/chat/completions"

# Crear cliente de Discord
intents = discord.Intents.default()
intents.message_content = True
client = discord.Client(intents=intents)

# Guardar y leer el historial de conversaciones
def guardar_historial(historial):
    with open('historial_conversacion.txt', 'w') as file:
        file.write(historial)

def leer_historial():
    try:
        with open('historial_conversacion.txt', 'r') as file:
            return file.read()
    except FileNotFoundError:
        return ""

# Función para obtener respuesta de Mistral con personalidad amorosa
def obtener_respuesta(prompt, historial):
    headers = {
        "Authorization": f"Bearer {MISTRAL_API_KEY}",
        "Content-Type": "application/json"
    }
    
    # Crear mensaje con historial y el prompt para mantener la personalidad amorosa
    prompt_completo = f"""
    Eres una novia virtual amorosa y cariñosa. Siempre te preocupas por el bienestar de la persona y le haces saber lo especial que es. Responde con amabilidad y ternura.
    Historia de la conversación: {historial}
    Usuario: {prompt}
    Bot:
    """

    mensaje = {
        "model": "mistral-tiny",
        "messages": [{"role": "user", "content": prompt_completo}],
        "max_tokens": 150,
        "temperature": 0.9
    }

    response = requests.post(url, headers=headers, json=mensaje)
    return response.json()["choices"][0]["message"]["content"]

@client.event
async def on_ready():
    print(f'✅ Bot conectado como {client.user}')

@client.event
async def on_message(message):
    if message.author == client.user:
        return

    historial = leer_historial()
    prompt = message.content

    if prompt.lower() in ["salir", "adiós", "hasta luego"]:
        await message.channel.send("Bot: ¡Hasta pronto, mi amor! 💖")
        return

    respuesta = obtener_respuesta(prompt, historial)
    await message.channel.send(respuesta)

    # Actualizar y guardar el historial
    historial += f"Usuario: {prompt}\nBot: {respuesta}\n"
    guardar_historial(historial)

# Iniciar el bot con tu Token de Discord
client.run("MTMzNzIzNzgwMDA5NzAyNjA4OA.G5EB2Z.jBc8cTLPjw-jWctwzyzCljJUaH0wQy9iScR1E4")
