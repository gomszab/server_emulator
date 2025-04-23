const invoke = window.__TAURI__.core.invoke;

const state = {
  fileContent: null
}
const stopServerButton = document.getElementById('stop-server');
const serverInfoDiv = document.getElementById('server-info');
const startServerButton = document.getElementById('start-server')
const hint = document.getElementById('hint')

const readerPromisify = (content) => {
  return new Promise((resolve) => {
    const reader = new FileReader(); 
    reader.onload = () => {
      resolve(reader.result)
    }
    reader.readAsText(content);
  })
}

document.getElementById('file-path').addEventListener('change', async (e) => {
  hint.style.display = 'none';
  const file = document.getElementById('file-path').files[0];
  const serverAddress =  "http://127.0.0.1:63003"
  const fileContent = await readerPromisify(file);
  state.fileContent = fileContent;
  try{
    const response = await invoke('load_configuration', { fileContent });
    serverInfoDiv.innerHTML = `
        <h3>Szerver elérése:</h3>
        <p>http://127.0.0.1:${response.port}</p> 
        <h3>Elérhető végpontok:</h3>
        <ul class="list-group">   
            ${response.endpoints.map(e => `<li class="list-group-item list-group-item-action list-group-item-primary">${e.method} <a>${serverAddress}${e.path}</a></li>`).join('')}
        </ul>
    `;
    hint.style.display = 'block';
    hint.innerHTML = 'A szerver jelenleg nem fut!';
  }catch(e){
    hint.style.display = 'block';
    hint.innerHTML = 'Nem megfelő a formátuma a konfigurációs fájlnak';
  }
  
})

document.getElementById('start-server').addEventListener('click', async (e) => {
  if(state.fileContent){
    try{
      await invoke('start_server', { fileContent: state.fileContent });
      stopServerButton.style.display = 'inline-block';
      startServerButton.style.display = 'none'; 
      hint.style.display = 'block';
      hint.innerHTML = 'A szerver jelenleg fut!';
    }catch(e){
      hint.style.display = 'block';
      hint.innerHTML = 'A szerver indulásakor hiba lépett fel!';
    }
  }else{
    hint.style.display = 'block';
    hint.innerHTML = 'Nincs betöltve konfiguráció';
  }
    
    
});

document.getElementById('stop-server').addEventListener('click', async (e) => {
  try{
    await invoke('stop_server');
    startServerButton.style.display = 'inline-block';
    stopServerButton.style.display = 'none';
    hint.style.display = 'block';
    hint.innerHTML = 'A szerver jelenleg nem fut!';
  }catch(e){
    hint.style.display = 'block';
    hint.innerHTML = 'Nem sikerült leállítani a szervert.';
  }
  
  
  
  
  
});



