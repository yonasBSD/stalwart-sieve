/*
 * Copyright (c) 2020-2023, Stalwart Labs Ltd.
 *
 * This file is part of the Stalwart Sieve Interpreter.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of
 * the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 * in the LICENSE file at the top-level directory of this distribution.
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * You can be released from the requirements of the AGPLv3 license by
 * purchasing a commercial license. Please contact licensing@stalw.art
 * for more details.
*/

use std::fmt::Display;

use phf::phf_map;

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
    EncodeUrl,
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
    Headers,
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
    NotifyMethodCapability,
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
    ValidNotifyMethod,
    Value,
    VirusTest,
    Zone,
}

pub(crate) static WORDS: phf::Map<&'static str, Word> = phf_map! {
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
    "encodeurl" => Word::EncodeUrl,
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
    "headers" => Word::Headers,
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
    "notify_method_capability" => Word::NotifyMethodCapability,
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
    "valid_notify_method" => Word::ValidNotifyMethod,
    "value" => Word::Value,
    "virustest" => Word::VirusTest,
    "zone" => Word::Zone,
};

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Word::AddFlag => f.write_str("addflag"),
            Word::AddHeader => f.write_str("addheader"),
            Word::Address => f.write_str("address"),
            Word::Addresses => f.write_str("addresses"),
            Word::All => f.write_str("all"),
            Word::AllOf => f.write_str("allof"),
            Word::AnyChild => f.write_str("anychild"),
            Word::AnyOf => f.write_str("anyof"),
            Word::Body => f.write_str("body"),
            Word::Break => f.write_str("break"),
            Word::ByMode => f.write_str("bymode"),
            Word::ByTimeAbsolute => f.write_str("bytimeabsolute"),
            Word::ByTimeRelative => f.write_str("bytimerelative"),
            Word::ByTrace => f.write_str("bytrace"),
            Word::Comparator => f.write_str("comparator"),
            Word::Contains => f.write_str("contains"),
            Word::Content => f.write_str("content"),
            Word::ContentType => f.write_str("contenttype"),
            Word::Convert => f.write_str("convert"),
            Word::Copy => f.write_str("copy"),
            Word::Count => f.write_str("count"),
            Word::Create => f.write_str("create"),
            Word::CurrentDate => f.write_str("currentdate"),
            Word::Date => f.write_str("date"),
            Word::Days => f.write_str("days"),
            Word::DeleteHeader => f.write_str("deleteheader"),
            Word::Detail => f.write_str("detail"),
            Word::Discard => f.write_str("discard"),
            Word::Domain => f.write_str("domain"),
            Word::Duplicate => f.write_str("duplicate"),
            Word::Else => f.write_str("else"),
            Word::ElsIf => f.write_str("elsif"),
            Word::Enclose => f.write_str("enclose"),
            Word::EncodeUrl => f.write_str("encodeurl"),
            Word::Envelope => f.write_str("envelope"),
            Word::Environment => f.write_str("environment"),
            Word::Ereject => f.write_str("ereject"),
            Word::Error => f.write_str("error"),
            Word::Exists => f.write_str("exists"),
            Word::ExtractText => f.write_str("extracttext"),
            Word::False => f.write_str("false"),
            Word::Fcc => f.write_str("fcc"),
            Word::FileInto => f.write_str("fileinto"),
            Word::First => f.write_str("first"),
            Word::Flags => f.write_str("flags"),
            Word::ForEveryPart => f.write_str("foreverypart"),
            Word::From => f.write_str("from"),
            Word::Global => f.write_str("global"),
            Word::Handle => f.write_str("handle"),
            Word::HasFlag => f.write_str("hasflag"),
            Word::Header => f.write_str("header"),
            Word::Headers => f.write_str("headers"),
            Word::If => f.write_str("if"),
            Word::Ihave => f.write_str("ihave"),
            Word::Importance => f.write_str("importance"),
            Word::Include => f.write_str("include"),
            Word::Index => f.write_str("index"),
            Word::Is => f.write_str("is"),
            Word::Keep => f.write_str("keep"),
            Word::Last => f.write_str("last"),
            Word::Length => f.write_str("length"),
            Word::List => f.write_str("list"),
            Word::LocalPart => f.write_str("localpart"),
            Word::Lower => f.write_str("lower"),
            Word::LowerFirst => f.write_str("lowerfirst"),
            Word::MailboxExists => f.write_str("mailboxexists"),
            Word::MailboxId => f.write_str("mailboxid"),
            Word::MailboxIdExists => f.write_str("mailboxidexists"),
            Word::Matches => f.write_str("matches"),
            Word::Message => f.write_str("message"),
            Word::Metadata => f.write_str("metadata"),
            Word::MetadataExists => f.write_str("metadataexists"),
            Word::Mime => f.write_str("mime"),
            Word::Name => f.write_str("name"),
            Word::Not => f.write_str("not"),
            Word::Notify => f.write_str("notify"),
            Word::NotifyMethodCapability => f.write_str("notify_method_capability"),
            Word::Once => f.write_str("once"),
            Word::Optional => f.write_str("optional"),
            Word::Options => f.write_str("options"),
            Word::OriginalZone => f.write_str("originalzone"),
            Word::Over => f.write_str("over"),
            Word::Param => f.write_str("param"),
            Word::Percent => f.write_str("percent"),
            Word::Personal => f.write_str("personal"),
            Word::QuoteRegex => f.write_str("quoteregex"),
            Word::QuoteWildcard => f.write_str("quotewildcard"),
            Word::Raw => f.write_str("raw"),
            Word::Redirect => f.write_str("redirect"),
            Word::Regex => f.write_str("regex"),
            Word::Reject => f.write_str("reject"),
            Word::RemoveFlag => f.write_str("removeflag"),
            Word::Replace => f.write_str("replace"),
            Word::Require => f.write_str("require"),
            Word::Ret => f.write_str("ret"),
            Word::Return => f.write_str("return"),
            Word::Seconds => f.write_str("seconds"),
            Word::ServerMetadata => f.write_str("servermetadata"),
            Word::ServerMetadataExists => f.write_str("servermetadataexists"),
            Word::Set => f.write_str("set"),
            Word::SetFlag => f.write_str("setflag"),
            Word::Size => f.write_str("size"),
            Word::SpamTest => f.write_str("spamtest"),
            Word::SpecialUse => f.write_str("specialuse"),
            Word::SpecialUseExists => f.write_str("specialuse_exists"),
            Word::Stop => f.write_str("stop"),
            Word::String => f.write_str("string"),
            Word::Subject => f.write_str("subject"),
            Word::Subtype => f.write_str("subtype"),
            Word::Text => f.write_str("text"),
            Word::True => f.write_str("true"),
            Word::Type => f.write_str("type"),
            Word::Under => f.write_str("under"),
            Word::UniqueId => f.write_str("uniqueid"),
            Word::Upper => f.write_str("upper"),
            Word::UpperFirst => f.write_str("upperfirst"),
            Word::User => f.write_str("user"),
            Word::Vacation => f.write_str("vacation"),
            Word::ValidExtList => f.write_str("valid_ext_list"),
            Word::ValidNotifyMethod => f.write_str("valid_notify_method"),
            Word::Value => f.write_str("value"),
            Word::VirusTest => f.write_str("virustest"),
            Word::Zone => f.write_str("zone"),
        }
    }
}
