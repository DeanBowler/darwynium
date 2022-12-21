import { useRef } from 'react';
import './App.css';

import { WorldInstance } from './native/pkg';

function App() {
  const world = useRef(WorldInstance.new());

  console.log(world);

  return (
    <div className="App">
      <h1>Darwynium</h1>
      {/* <h2>Welcome to {world.current.get_name()}</h2> */}
      <div className="card">
        <button onClick={() => world.current.greet()}>hollar</button>
      </div>
    </div>
  );
}

export default App;
