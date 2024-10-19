import { useState } from 'react';
import reactLogo from './assets/react.svg';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

function App() {
    const [greetMsg, setGreetMsg] = useState('');
    const [name, setName] = useState('');
    const [url, setUrl] = useState('');

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        setGreetMsg(await invoke('greet', { name }));
    }

    async function get_web_page() {
        const body = (await invoke('get_web_page', { url })) as string;
        setGreetMsg(body);
    }

    return (
        <main className='container'>
            <h1>Welcome to Mashu</h1>

            <p>Click on the Tauri, Vite, and React logos to learn more.</p>

            <form
                className='row form-spacing'
                onSubmit={(e) => {
                    e.preventDefault();
                    greet();
                }}
            >
                <input id='greet-input' onChange={(e) => setName(e.currentTarget.value)} placeholder='Enter a name...' />
                <button type='submit'>Greet</button>
            </form>

            <form
                className='row form-spacing'
                onSubmit={(e) => {
                    e.preventDefault();
                    get_web_page();
                }}
            >
                <input id='greet-input' onChange={(e) => setUrl(e.currentTarget.value)} placeholder='https://v2.tauri.app/' />
                <button type='submit'>get web page</button>
            </form>

            <button type='button' onClick={() => setGreetMsg('')}>Clear</button>

            <p>{greetMsg}</p>
        </main>
    );
}

export default App;
