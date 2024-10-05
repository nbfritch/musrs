import './app.css';
import { store } from './app/store';
import { Player } from './features/player/Player';
import { Provider } from 'react-redux';

export const App = () => {
  return (
    <Provider store={store}>
      <Player />
    </Provider>
  )
};
