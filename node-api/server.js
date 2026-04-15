const express = require('express');
const dotenv = require('dotenv');
dotenv.config();

const app = express();

app.set('port', process.env.PORT || 3000);
app.set('host', process.env.HOST || 'localhost');

app.use(express.json({
    limit: '1mb'
}));

app.get('/', (req, res) => {
    res.status(200).json({ message: 'hell yeah brother' });
});

app.post('/api/v1/users', (req, res) => {
    const body = req.body;
    const email = body.email;
    const password = body.password;

    console.info(`User ${email} logged in with password ${password}`);

    res.status(200).json({ message: `OK` });
});

app.listen(app.get('port'), app.get('host'), 0, () => {
    console.info(`Server is running at ${app.get('host')}:${app.get('port')}! Say hi!`);
});