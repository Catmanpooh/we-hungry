pub const CARD_GET_ALL_CLASS: &str ="rounded-lg border bg-card text-card-foreground shadow-sm";
pub const CARD_CLASS: &str ="rounded-lg border bg-card text-card-foreground shadow-sm";
pub const CARD_HEADER_CLASS: &str = "flex flex-col space-y-1.5 p-6";
pub const CARD_TITLE_CLASS: &str = "text-2xl font-semibold leading-none tracking-tight";
pub const CARD_DESCRIPTION_CLASS: &str = "text-sm text-muted-foreground";
pub const CARD_CONTENT_CLASS: &str = "p-6 pt-0";
pub const CARD_FOOTER_CLASS: &str = "flex items-center my-6";
pub const ERROR_CLASS: &str = "text-sm text-red-400 my-6";

pub const LABEL_INPUT_DIV_CLASS: &str = "flex flex-col space-y-1.5";
pub const LABEL_CLASS: &str =
    "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70";

pub const INPUT_CLASS: &str = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm 
    ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground 
    placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring 
    focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

pub const TEXTAREA_CLASS: &str = "flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2
    text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2
    focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

pub const BUTTON_CLASS: &str = " bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2 inline-flex
    items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background 
    transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 
    disabled:pointer-events-none disabled:opacity-50";
