use tora::{ReadEnum, ReadStruct, WriteEnum, WriteStruct};

/// A result returned by the client.
pub type ClientResult<T> = Result<T, String>;

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
    Command(String, Vec<String>),

    /// Spawns a thread and executes the [Command](Self::Command) operation.
    ThreadedCommand(Vec<String>),

    /// An empty operation used to measure the send and response time of the host-client connection.
    Ping,
}

/// A response to an [Operation].
#[derive(ReadEnum, WriteEnum)]
pub enum Response {
    /// The response to the upload operation.
    Upload(ClientResult<()>),

    /// The response to the download operation.
    Download(ClientResult<()>),

    /// The response to the ping operation.
    Pong,
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
