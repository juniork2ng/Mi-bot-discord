const { makeWASocket, useSingleFileAuthState } = require('@whiskeysockets/baileys');

// Ruta para el archivo de autenticación
const { state, saveState } = useSingleFileAuthState('auth_info.json');

// Crear el socket
const sock = makeWASocket({
  auth: state,
  printQRInTerminal: true, // Esto imprimirá el QR en la terminal para que puedas escanearlo
});

// Guardar el estado de autenticación
sock.ev.on('auth-state.update', saveState);

// Event listener para conexión
sock.ev.on('connection.update', (update) => {
  const { connection, lastDisconnect } = update;
  if (connection === 'close') {
    console.log('conexión cerrada', lastDisconnect.error);
  } else if (connection === 'open') {
    console.log('conexión abierta');
  }
});
