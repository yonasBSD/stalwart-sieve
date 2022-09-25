pub mod lexer;
use phf::phf_map;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Token {
    CurlyOpen,
    CurlyClose,
    BracketOpen,
    BracketClose,
    ParenthesisOpen,
    ParenthesisClose,
    Comma,
    Semicolon,
    String(Vec<u8>),
    Number(usize),
    Identifier(Word),
    Tag(Word),
    Invalid(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Word {
    AddFlag,
    AddHeader,
    Address,
    Addresses,
    All,
    AllOf,
    AnyChild,
    AnyOf,
    Body,
    Break,
    ByMode,
    ByTimeAbsolute,
    ByTimeRelative,
    ByTrace,
    Comparator,
    Contains,
    Content,
    ContentType,
    Convert,
    Copy,
    Count,
    Create,
    CurrentDate,
    Date,
    Days,
    DeleteHeader,
    Detail,
    Discard,
    Domain,
    Duplicate,
    Else,
    ElsIf,
    Enclose,
    Envelope,
    Environment,
    Ereject,
    Error,
    Exists,
    ExtractText,
    False,
    Fcc,
    FileInto,
    First,
    Flags,
    ForEveryPart,
    From,
    Global,
    Handle,
    HasFlag,
    Header,
    If,
    Ihave,
    Importance,
    Include,
    Index,
    Is,
    Keep,
    Last,
    Length,
    List,
    LocalPart,
    Lower,
    LowerFirst,
    MailboxExists,
    MailboxId,
    MailboxIdExists,
    Matches,
    Message,
    Metadata,
    MetadataExists,
    Mime,
    Name,
    Not,
    Notify,
    Once,
    Optional,
    Options,
    OriginalZone,
    Over,
    Param,
    Percent,
    Personal,
    QuoteRegex,
    QuoteWildcard,
    Raw,
    Redirect,
    Regex,
    Reject,
    RemoveFlag,
    Replace,
    Require,
    Ret,
    Return,
    Seconds,
    ServerMetadata,
    ServerMetadataExists,
    Set,
    SetFlag,
    Size,
    SpamTest,
    SpecialUse,
    SpecialUseExists,
    Stop,
    String,
    Subject,
    Subtype,
    Text,
    True,
    Type,
    Under,
    UniqueId,
    Upper,
    UpperFirst,
    User,
    Vacation,
    ValidExtList,
    Value,
    VirusTest,
    Zone,
}

static WORDS: phf::Map<&'static str, Word> = phf_map! {
    "addflag" => Word::AddFlag,
    "addheader" => Word::AddHeader,
    "address" => Word::Address,
    "addresses" => Word::Addresses,
    "all" => Word::All,
    "allof" => Word::AllOf,
    "anychild" => Word::AnyChild,
    "anyof" => Word::AnyOf,
    "body" => Word::Body,
    "break" => Word::Break,
    "bymode" => Word::ByMode,
    "bytimeabsolute" => Word::ByTimeAbsolute,
    "bytimerelative" => Word::ByTimeRelative,
    "bytrace" => Word::ByTrace,
    "comparator" => Word::Comparator,
    "contains" => Word::Contains,
    "content" => Word::Content,
    "contenttype" => Word::ContentType,
    "convert" => Word::Convert,
    "copy" => Word::Copy,
    "count" => Word::Count,
    "create" => Word::Create,
    "currentdate" => Word::CurrentDate,
    "date" => Word::Date,
    "days" => Word::Days,
    "deleteheader" => Word::DeleteHeader,
    "detail" => Word::Detail,
    "discard" => Word::Discard,
    "domain" => Word::Domain,
    "duplicate" => Word::Duplicate,
    "else" => Word::Else,
    "elsif" => Word::ElsIf,
    "enclose" => Word::Enclose,
    "envelope" => Word::Envelope,
    "environment" => Word::Environment,
    "ereject" => Word::Ereject,
    "error" => Word::Error,
    "exists" => Word::Exists,
    "extracttext" => Word::ExtractText,
    "false" => Word::False,
    "fcc" => Word::Fcc,
    "fileinto" => Word::FileInto,
    "first" => Word::First,
    "flags" => Word::Flags,
    "foreverypart" => Word::ForEveryPart,
    "from" => Word::From,
    "global" => Word::Global,
    "handle" => Word::Handle,
    "hasflag" => Word::HasFlag,
    "header" => Word::Header,
    "if" => Word::If,
    "ihave" => Word::Ihave,
    "importance" => Word::Importance,
    "include" => Word::Include,
    "index" => Word::Index,
    "is" => Word::Is,
    "keep" => Word::Keep,
    "last" => Word::Last,
    "length" => Word::Length,
    "list" => Word::List,
    "localpart" => Word::LocalPart,
    "lower" => Word::Lower,
    "lowerfirst" => Word::LowerFirst,
    "mailboxexists" => Word::MailboxExists,
    "mailboxid" => Word::MailboxId,
    "mailboxidexists" => Word::MailboxIdExists,
    "matches" => Word::Matches,
    "message" => Word::Message,
    "metadata" => Word::Metadata,
    "metadataexists" => Word::MetadataExists,
    "mime" => Word::Mime,
    "name" => Word::Name,
    "not" => Word::Not,
    "notify" => Word::Notify,
    "once" => Word::Once,
    "optional" => Word::Optional,
    "options" => Word::Options,
    "originalzone" => Word::OriginalZone,
    "over" => Word::Over,
    "param" => Word::Param,
    "percent" => Word::Percent,
    "personal" => Word::Personal,
    "quoteregex" => Word::QuoteRegex,
    "quotewildcard" => Word::QuoteWildcard,
    "raw" => Word::Raw,
    "redirect" => Word::Redirect,
    "regex" => Word::Regex,
    "reject" => Word::Reject,
    "removeflag" => Word::RemoveFlag,
    "replace" => Word::Replace,
    "require" => Word::Require,
    "ret" => Word::Ret,
    "return" => Word::Return,
    "seconds" => Word::Seconds,
    "servermetadata" => Word::ServerMetadata,
    "servermetadataexists" => Word::ServerMetadataExists,
    "set" => Word::Set,
    "setflag" => Word::SetFlag,
    "size" => Word::Size,
    "spamtest" => Word::SpamTest,
    "specialuse" => Word::SpecialUse,
    "specialuse_exists" => Word::SpecialUseExists,
    "stop" => Word::Stop,
    "string" => Word::String,
    "subject" => Word::Subject,
    "subtype" => Word::Subtype,
    "text" => Word::Text,
    "true" => Word::True,
    "type" => Word::Type,
    "under" => Word::Under,
    "uniqueid" => Word::UniqueId,
    "upper" => Word::Upper,
    "upperfirst" => Word::UpperFirst,
    "user" => Word::User,
    "vacation" => Word::Vacation,
    "valid_ext_list" => Word::ValidExtList,
    "value" => Word::Value,
    "virustest" => Word::VirusTest,
    "zone" => Word::Zone,
};
