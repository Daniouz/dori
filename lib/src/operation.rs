use tora::{ReadEnum, ReadStruct, WriteEnum, WriteStruct};

/// An error returned when a packet could not be read.
///
/// # Variants
///
/// - BadPacket: the packet was written or received incorrectly
/// - UnknownOperation: the operation is not recognized by this version of dori
#[derive(Debug, ReadEnum, WriteEnum)]
pub enum ReadError {
    /// The packet was written or received incorrectly.
    BadPacket,

    /// The operation is not recognized by this version of dori.
    UnknownOperation,
}

/// An operation that is sent by the host and executed on the client.
///
/// # Supported Operations
///
/// - Upload: uploads a file to the client
/// - Download: downloads a file from the client
/// - Command: executes a shell command and awaits the completion and output as a response
/// - ThreadedCommand: spawns a thread and executes the Command operation
/// - Ping: an empty operation used to measure the send and response time of the connection
#[derive(ReadEnum, WriteEnum)]
pub enum Operation {
    /// Uploads a file to the client.
    Upload(FileTransferOperation),

    /// Downloads a file from the client.
    Download(FileTransferOperation),

    /// Executes a shell command and awaits the completion and output as a response.
    Command(Vec<String>),

    /// Spawns a thread and executes the [Command](Self::Command) operation.
    ThreadedCommand(Vec<String>),

    /// An empty operation used to measure the send and response time of the host-client connection.
    Ping,
}

/// A response to an [Operation].
#[derive(ReadEnum, WriteEnum)]
pub enum Response {
    /// The response to the upload operation.
    Upload(Result<(), ClientError>),

    /// The response to the download operation.
    Download(Result<(), ClientError>),

    /// The response to the ping operation.
    Pong,
}

#[derive(ReadStruct, WriteStruct)]
pub struct ClientError {
    status: i32,
    message: Option<String>,
}

impl ClientError {
    /// Returns the status of the error.
    pub fn status(&self) -> i32 {
        self.status
    }

    /// Returns the error message, if present.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

/// Operation in which a file is transferred to or from the client.
#[derive(WriteStruct, ReadStruct)]
pub struct FileTransferOperation {
    path: String,
    content: Vec<u8>,
}

impl FileTransferOperation {
    /// Returns the path to the file destination.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns the file content as a slice.
    pub fn content(&self) -> &[u8] {
        &self.content
    }

    /// Instantiates a new FileTransferOperation.
    ///
    /// # Parameters
    ///
    /// - path: The path to the file destination.
    /// - content: The file content.
    pub const fn new(path: String, content: Vec<u8>) -> Self {
        Self { path, content }
    }
}
