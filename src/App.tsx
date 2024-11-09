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
        const raw = `
            flowchart TD
                ct001["name: head,  type: control"]
                ct002["name: end,   type: control"]
                ct003["name: sleep, type: control, para: { ms:5000 }"]
                op001["name: init_web, type: operate, para: { url:"http://localhost:9515" }"]
                op002["name: open_web, type: operate, para: { url:'www.google.com' }"]
                de001["name: concurrent,   type: decorate"] 
                op003["name: input_string, type: operate, para: { component:'', input:'red panda' }"]
                op004["name: input_string, type: operate, para: { component:'', input:'very cute' }"]
                op005["name: press_button, type: operate, para: { component:'' }"]

                ct001 -->|success| op001
                op001 -->|success| op002
                op002 -->|success| de001
                op005 -->|success| ct003
                ct003 -->|success| ct002
                de001 -->|success| op005

                op001 -->|fail| ct002
                op002 -->|fail| ct002
                op005 -->|fail| ct002
                de001 -->|fail| ct002

                de001 -->|decorate| op003
                de001 -->|decorate| op004
        `;

        try {
            const script = btoa(raw);
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
