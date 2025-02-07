const makeWASocket = require("@whiskeysockets/baileys").default;
const { generateWAMessageFromContent, proto } = require("@whiskeysockets/baileys");
const { Boom } = require("@hapi/boom");
const qrcode = require("qrcode-terminal");
const axios = require("axios");

async function startBot() {
    const sock = makeWASocket({ printQRInTerminal: true });

    sock.ev.on("connection.update", (update) => {
        if (update.qr) {
            qrcode.generate(update.qr, { small: true });
        } else if (update.connection === "open") {
            console.log("✅ Bot conectado a WhatsApp");
        } else if (update.connection === "close") {
            console.log("❌ Conexión cerrada, reconectando...");
            startBot();
        }
    });

    sock.ev.on("messages.upsert", async (msg) => {
        const m = msg.messages[0];
        if (!m.message || m.key.fromMe) return;

        const chatId = m.key.remoteJid;
        const texto = m.message.conversation || m.message.extendedTextMessage?.text;

        if (!texto) return;

        // Conectando con ChatGPT para generar respuestas
        const response = await axios.post("https://api.openai.com/v1/chat/completions", {
            model: "gpt-3.5-turbo",
            messages: [{ role: "user", content: texto }]
        }, {
            headers: {
                "Authorization": `Bearer TU_CLAVE_DE_OPENAI`,
                "Content-Type": "application/json"
            }
        });

        const respuesta = response.data.choices[0].message.content;

        await sock.sendMessage(chatId, { text: respuesta });
    });
}

startBot();
