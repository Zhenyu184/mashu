class ContentManager {
    private static instance: ContentManager;
    private content: string;

    private constructor() {
        this.content = `
            flowchart TD
                ct001["name: head,  type: control"]
                ct002["name: end,   type: control"]

                ct001 -->|success| ct002
            `;
    }

    public static getInstance(): ContentManager {
        if (!ContentManager.instance) {
            ContentManager.instance = new ContentManager();
        }
        return ContentManager.instance;
    }

    public getContent(): string {
        return this.content;
    }

    public registerNode(id: string, name: string, type: string, para: string): void {
        const addComtent = `${id}["name: ${name}, type: ${type}, para: ${para}"]`;
    }

    public unregisterNode(id: string): void {}

    public setContent(newContent: string): void {
        this.content = newContent;
    }
}

const contentManager = ContentManager.getInstance();
export default contentManager;
