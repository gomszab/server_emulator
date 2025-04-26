const form = document.createElement('form');
document.body.appendChild(form);
const inputName = document.createElement('input');
inputName.type = 'text';
form.appendChild(inputName)

const inputNumber = document.createElement('input');
inputNumber.type = 'number';
form.appendChild(inputNumber)

const button = document.createElement('button');
button.textContent = 'Hozzaad';
form.appendChild(button);
form.addEventListener('submit', (e) => {
    e.preventDefault();
    fetch('http://127.0.0.1:63013/fruits', {
        method: 'POST', 
        headers: {
            'Content-Type' : 'application/json'
        },
        body: JSON.stringify(
            {  // Properly stringify the object
                name: inputName.value,
                price: Number(inputNumber.value)
            }
        ) 
    }).then(response => response.json()).then(value => {
        console.log(value);
    }).catch(e => console.log(e))
})
// http://127.0.0.1:63003/items