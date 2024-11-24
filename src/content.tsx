class ContentManager {
    private static instance: ContentManager;
    private content: string;

    private constructor() {
        this.content = '';
    }

    public static getInstance(): ContentManager {
        if (!ContentManager.instance) {
            ContentManager.instance = new ContentManager();
        }
        return ContentManager.instance;
    }

    public setContent(newContent: string): void {
        this.content = newContent;
    }

    public getContent(): string {
        return this.content;
    }
}

const contentManager = ContentManager.getInstance();
export default contentManager;
