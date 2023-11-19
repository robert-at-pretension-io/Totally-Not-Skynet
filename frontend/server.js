const express = require('express');
const path = require('path');
const app = express();

// Serve static files from the 'public' folder
app.use(express.static(path.join(__dirname, 'public')));

const PORT = process.env.PORT || 4200;

app.listen(PORT, () => {
    console.log(`Server is running on port ${PORT}`);
});
