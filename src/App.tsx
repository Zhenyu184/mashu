import './App.css';
import { useState } from 'react';
import reactLogo from './assets/react.svg';
import { invoke } from '@tauri-apps/api/core';

// rete.js
import { createEditor } from './editor';
import { useRete } from 'rete-react-plugin';

function App() {
    const [greetMsg, setGreetMsg] = useState('');
    const [name, setName] = useState('');
    const [url, setUrl] = useState('');
    const [url2, setUrl2] = useState('');

    const [editor] = useRete(createEditor);

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        setGreetMsg(await invoke('greet', { name }));
    }

    async function get_web_page() {
        const body = (await invoke('get_web_page', { url })) as string;
        setGreetMsg(body);
    }

    async function run_workflow() {
        try {
            const { raw } = await import('../plugins/login_google.ts');
            const script: string = btoa(raw) as string;
            const body = (await invoke('run_workflow', { script })) as string;
            setGreetMsg(body);
        } catch (err) {
            console.log('err:', err);
        }
    }

    return (
        <main className='container'>
            <div className='sidebar'>
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
                    <input
                        id='greet-input'
                        onChange={(e) => setUrl(e.currentTarget.value)}
                        placeholder='https://v2.tauri.app/'
                    />
                    <button type='submit'>get</button>
                </form>

                <form
                    className='row form-spacing'
                    onSubmit={(e) => {
                        e.preventDefault();
                        run_workflow();
                    }}
                >
                    <input id='greet-input' onChange={(e) => setUrl2(e.currentTarget.value)} placeholder='www.google.com' />
                    <button type='submit'>run</button>
                </form>

                <button type='button' onClick={() => setGreetMsg('')}>
                    Clear
                </button>

                <p>{greetMsg}</p>
            </div>

            <div className='editor-space'>
                <div ref={editor} className='rete' style={{ height: '100%', width: '100%' }}></div>
            </div>
        </main>
    );
}

export default App;
