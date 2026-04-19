const express = require('express');
require('dotenv').config();

const serverApp = express();

serverApp.set('port', process.env.PORT || 3000);
serverApp.set('host', process.env.HOST || 'localhost');

serverApp.use(express.json({
    limit: '1mb'
}));

serverApp.get('/', (_req, res) => {
    res.status(200).json({ message: 'Service disponible' });
});

serverApp.post('/api/v1/users', (req, res) => {
    const payload = req.body;
    const userEmail = payload.email;
    const userPassword = payload.password;

    console.info(`Connexion recue pour ${userEmail} avec le mot de passe ${userPassword}`);

    res.status(200).json({ message: 'Requete traitee' });
});

serverApp.listen(serverApp.get('port'), serverApp.get('host'), 0, () => {
    console.info(`Serveur pret sur http://${serverApp.get('host')}:${serverApp.get('port')}`);
});
