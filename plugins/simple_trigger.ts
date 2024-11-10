export const raw = `
    flowchart TD
        ct001["name: head,  type: control"]
        ct002["name: end,   type: control"]
        ct004["name: timing, type: control, para: { cron:'0 32 21 * * * *' }"]
        ct003["name: sleep, type: control, para: { ms:'10000' }"]
        op001["name: init_web, type: operate, para: { url:'http://localhost:9515' }"]
        op002["name: open_web, type: operate, para: { url:'www.google.com' }"]
        op003["name: input_string, type: operate, para: { component:'q', input:'red panda' }"]
        op004["name: summit, type: operate, para: { component:'q' }"]

        ct001 -->|success| ct004
        ct004 -->|success| op001
        op001 -->|success| op002
        op002 -->|success| op003
        op003 -->|success| op004

        op001 -->|fail| ct002
        op002 -->|fail| ct002
        op003 -->|fail| ct003

        op004 -->|always| ct003
        ct003 -->|always| ct002
`;
