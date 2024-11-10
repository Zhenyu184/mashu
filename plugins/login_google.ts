export const raw = `
    flowchart TD
        ct001["name: head,  type: control"]
        ct002["name: end,   type: control"]
        op001["name: init_web, type: operate, para: { url:'http://localhost:9515' }"]
        op002["name: open_web, type: operate, para: { url:'https://accounts.google.com/' }"]
        op003["name: input_string, type: operate, para: { component:'identifierId', input:'zzy1120126@gmail.com' }"]
        op004["name: press_button, type: operate, para: { component:'identifierNext' }"]
        ct003["name: sleep, type: control, para: { ms:'1000' }"]

        ct001 -->|success| op001
        op001 -->|success| op002
        op002 -->|success| op003

        op001 -->|fail| ct002
        op002 -->|fail| ct002

        op003 -->|always| op004
        op004 -->|always| ct003
        ct003 -->|always| ct002
`;
