const express = require('express');
const bodyParser = require('body-parser');
const app = express();
app.use(bodyParser.urlencoded({ extended: true }));
const port = 3000

app.get('/', (req, res) => {
    console.log('get')
  res.send('Hello World!')
})

app.post('/', (req, res) => {
    console.log('post')
    console.log(req.headers)
    console.log(req.body)
    res.send('Post Hello World!')
  })
  

app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})