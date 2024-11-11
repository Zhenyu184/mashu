import './App.css';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

// rete.js
import { createEditor } from './editor';
import { useRete } from 'rete-react-plugin';

function App() {
    const [greetMsg, setGreetMsg] = useState('');
    const [name, setName] = useState('');

    const [editor] = useRete(createEditor);

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        setGreetMsg(await invoke('greet', { name }));
    }

    async function run_workflow(pluginPath: string) {
        try {
            const { raw } = await import(pluginPath);
            const script: string = btoa(raw) as string;
            const body = (await invoke('run_workflow', { script })) as string;
            setGreetMsg(body);
        } catch (err) {
            console.log('err:', err);
        }
    }

    async function stop_workflow() {
        setGreetMsg('the function has not been implemented yet');
    }

    async function clear_msg() {
        setGreetMsg('');
    }

    return (
        <main className='container'>
            <div className='sidebar'>
                <h1>Welcome to Mashu</h1>

                <p>
                    The purpose of this project is to make web crawling tools graphical. Mashu is made in memory of a cute baby
                    red panda.
                </p>

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

                <div className='button-container'>
                    <button type='button' onClick={(_) => run_workflow('../plugins/login_google.ts')}>
                        Run
                    </button>
                    <button type='button' onClick={(_) => stop_workflow()}>
                        Stop
                    </button>
                    <button type='button' onClick={(_) => clear_msg()}>
                        Clear
                    </button>
                </div>

                <p>{greetMsg}</p>
            </div>

            <div className='editor-space'>
                <div ref={editor} className='rete' style={{ height: '100%', width: '100%' }}></div>
            </div>
        </main>
    );
}

export default App;
