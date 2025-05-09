const express = require('express');
const path = require('path');
const app = express();

app.set('views', path.join(__dirname, 'views'));
app.set('view engine', 'pug');
app.use(express.static(path.join(__dirname, 'public')));

app.get('/', (req, res) => {
    res.render('layout');
});

app.get('/map_editor', (req, res) => {
    res.sendFile(path.join(__dirname, 'public', 'map_editor.html'));
});

app.listen(3000, () => {
    console.log('Server running on http://localhost:3000');
});
