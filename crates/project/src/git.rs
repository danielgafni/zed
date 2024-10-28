use crate::buffer_store::BufferStore;
use crate::worktree_store::{WorktreeStore, WorktreeStoreEvent};
use crate::{Project, ProjectPath};
use anyhow::Context as _;
use client::ProjectId;
use futures::channel::{mpsc, oneshot};
use futures::StreamExt as _;
use git::{
    repository::{GitRepository, RepoPath},
    status::{GitSummary, TrackedSummary},
};
use gpui::{
    App, AppContext, Context, Entity, EventEmitter, SharedString, Subscription, Task, WeakEntity,
};
use language::{Buffer, LanguageRegistry};
use rpc::{proto, AnyProtoClient};
use settings::WorktreeId;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use text::BufferId;
use util::{maybe, ResultExt};
use worktree::{ProjectEntryId, RepositoryEntry, StatusEntry};

pub struct GitState {
    project_id: Option<ProjectId>,
    client: Option<AnyProtoClient>,
    repositories: Vec<Entity<Repository>>,
    active_index: Option<usize>,
    update_sender: mpsc::UnboundedSender<(Message, oneshot::Sender<anyhow::Result<()>>)>,
    _subscription: Subscription,
}

pub struct Repository {
    commit_message_buffer: Option<Entity<Buffer>>,
    git_state: WeakEntity<GitState>,
    pub worktree_id: WorktreeId,
    pub repository_entry: RepositoryEntry,
    pub git_repo: GitRepo,
<<<<<<< HEAD
    pub merge_message: Option<String>,
    update_sender: mpsc::UnboundedSender<(Message, oneshot::Sender<Result<()>>)>,
||||||| parent of 673f7e3e26 (cachix and fontdb)
    update_sender: mpsc::UnboundedSender<(Message, oneshot::Sender<Result<()>>)>,
=======
    update_sender: mpsc::UnboundedSender<(Message, oneshot::Sender<anyhow::Result<()>>)>,
>>>>>>> 673f7e3e26 (cachix and fontdb)
}

#[derive(Clone)]
pub enum GitRepo {
    Local(Arc<dyn GitRepository>),
    Remote {
        project_id: ProjectId,
        client: AnyProtoClient,
        worktree_id: WorktreeId,
        work_directory_id: ProjectEntryId,
    },
}

enum Message {
    Commit {
        git_repo: GitRepo,
        message: SharedString,
        name_and_email: Option<(SharedString, SharedString)>,
    },
<<<<<<< HEAD
    Reset {
        repo: GitRepo,
        commit: SharedString,
        reset_mode: ResetMode,
    },
    CheckoutFiles {
        repo: GitRepo,
        commit: SharedString,
        paths: Vec<RepoPath>,
    },
||||||| parent of 673f7e3e26 (cachix and fontdb)
    Reset {
        repo: GitRepo,
        commit: SharedString,
        reset_mode: ResetMode,
    },
=======
>>>>>>> 673f7e3e26 (cachix and fontdb)
    Stage(GitRepo, Vec<RepoPath>),
    Unstage(GitRepo, Vec<RepoPath>),
}

pub enum GitEvent {
    ActiveRepositoryChanged,
    FileSystemUpdated,
    GitStateUpdated,
}

impl EventEmitter<GitEvent> for GitState {}

impl GitState {
    pub fn new(
        worktree_store: &Entity<WorktreeStore>,
        client: Option<AnyProtoClient>,
        project_id: Option<ProjectId>,
        cx: &mut Context<'_, Self>,
    ) -> Self {
        let update_sender = Self::spawn_git_worker(cx);
        let _subscription = cx.subscribe(worktree_store, Self::on_worktree_store_event);

        GitState {
            project_id,
            client,
            repositories: Vec::new(),
            active_index: None,
            update_sender,
            _subscription,
        }
    }

<<<<<<< HEAD
    pub fn init(client: &AnyProtoClient) {
        client.add_entity_request_handler(Self::handle_stage);
        client.add_entity_request_handler(Self::handle_unstage);
        client.add_entity_request_handler(Self::handle_commit);
        client.add_entity_request_handler(Self::handle_reset);
        client.add_entity_request_handler(Self::handle_show);
        client.add_entity_request_handler(Self::handle_checkout_files);
        client.add_entity_request_handler(Self::handle_open_commit_message_buffer);
        client.add_entity_request_handler(Self::handle_set_index_text);
    }

||||||| parent of 673f7e3e26 (cachix and fontdb)
    pub fn init(client: &AnyProtoClient) {
        client.add_entity_request_handler(Self::handle_stage);
        client.add_entity_request_handler(Self::handle_unstage);
        client.add_entity_request_handler(Self::handle_commit);
        client.add_entity_request_handler(Self::handle_reset);
        client.add_entity_request_handler(Self::handle_show);
        client.add_entity_request_handler(Self::handle_open_commit_message_buffer);
        client.add_entity_request_handler(Self::handle_set_index_text);
    }

=======
>>>>>>> 673f7e3e26 (cachix and fontdb)
    pub fn active_repository(&self) -> Option<Entity<Repository>> {
        self.active_index
            .map(|index| self.repositories[index].clone())
    }

    fn on_worktree_store_event(
        &mut self,
        worktree_store: Entity<WorktreeStore>,
        event: &WorktreeStoreEvent,
        cx: &mut Context<'_, Self>,
    ) {
        let mut new_repositories = Vec::new();
        let mut new_active_index = None;
        let this = cx.weak_entity();
        let client = self.client.clone();
        let project_id = self.project_id;

        worktree_store.update(cx, |worktree_store, cx| {
            for worktree in worktree_store.worktrees() {
                worktree.update(cx, |worktree, cx| {
                    let snapshot = worktree.snapshot();
                    for repo in snapshot.repositories().iter() {
                        let git_data = worktree
                            .as_local()
                            .and_then(|local_worktree| local_worktree.get_local_repo(repo))
                            .map(|local_repo| {
                                (
                                    GitRepo::Local(local_repo.repo().clone()),
                                    local_repo.merge_message.clone(),
                                )
                            })
                            .or_else(|| {
                                let client = client.clone()?;
                                let project_id = project_id?;
                                Some((
                                    GitRepo::Remote {
                                        project_id,
                                        client,
                                        worktree_id: worktree.id(),
                                        work_directory_id: repo.work_directory_id(),
                                    },
                                    None,
                                ))
                            });
                        let Some((git_repo, merge_message)) = git_data else {
                            continue;
                        };
                        let worktree_id = worktree.id();
                        let existing =
                            self.repositories
                                .iter()
                                .enumerate()
                                .find(|(_, existing_handle)| {
                                    existing_handle.read(cx).id()
                                        == (worktree_id, repo.work_directory_id())
                                });
                        let handle = if let Some((index, handle)) = existing {
                            if self.active_index == Some(index) {
                                new_active_index = Some(new_repositories.len());
                            }
                            // Update the statuses and merge message but keep everything else.
                            let existing_handle = handle.clone();
                            existing_handle.update(cx, |existing_handle, cx| {
                                existing_handle.repository_entry = repo.clone();
                                if matches!(git_repo, GitRepo::Local { .. })
                                    && existing_handle.merge_message != merge_message
                                {
                                    if let (Some(merge_message), Some(buffer)) =
                                        (&merge_message, &existing_handle.commit_message_buffer)
                                    {
                                        buffer.update(cx, |buffer, cx| {
                                            if buffer.is_empty() {
                                                buffer.set_text(merge_message.as_str(), cx);
                                            }
                                        })
                                    }
                                    existing_handle.merge_message = merge_message;
                                }
                            });
                            existing_handle
                        } else {
                            cx.new(|_| Repository {
                                git_state: this.clone(),
                                worktree_id,
                                repository_entry: repo.clone(),
                                git_repo,
                                update_sender: self.update_sender.clone(),
                                merge_message,
                                commit_message_buffer: None,
                            })
                        };
                        new_repositories.push(handle);
                    }
                })
            }
        });

        if new_active_index == None && new_repositories.len() > 0 {
            new_active_index = Some(0);
        }

        self.repositories = new_repositories;
        self.active_index = new_active_index;

        match event {
            WorktreeStoreEvent::WorktreeUpdatedGitRepositories(_) => {
                cx.emit(GitEvent::GitStateUpdated);
            }
            _ => {
                cx.emit(GitEvent::FileSystemUpdated);
            }
        }
    }

    pub fn all_repositories(&self) -> Vec<Entity<Repository>> {
        self.repositories.clone()
    }

    fn spawn_git_worker(
        cx: &mut Context<'_, GitState>,
    ) -> mpsc::UnboundedSender<(Message, oneshot::Sender<anyhow::Result<()>>)> {
        let (update_sender, mut update_receiver) =
            mpsc::unbounded::<(Message, oneshot::Sender<anyhow::Result<()>>)>();
        cx.spawn(|_, cx| async move {
            while let Some((msg, respond)) = update_receiver.next().await {
                let result = cx
                    .background_executor()
                    .spawn(Self::process_git_msg(msg))
                    .await;
                respond.send(result).ok();
            }
        })
        .detach();
        update_sender
    }

    async fn process_git_msg(msg: Message) -> Result<(), anyhow::Error> {
        match msg {
            Message::Stage(repo, paths) => {
                match repo {
                    GitRepo::Local(repo) => repo.stage_paths(&paths)?,
                    GitRepo::Remote {
                        project_id,
                        client,
                        worktree_id,
                        work_directory_id,
                    } => {
                        client
                            .request(proto::Stage {
                                project_id: project_id.0,
                                worktree_id: worktree_id.to_proto(),
                                work_directory_id: work_directory_id.to_proto(),
                                paths: paths
                                    .into_iter()
                                    .map(|repo_path| repo_path.to_proto())
                                    .collect(),
                            })
                            .await
                            .context("sending stage request")?;
                    }
                }
                Ok(())
            }
<<<<<<< HEAD
            Message::Reset {
                repo,
                commit,
                reset_mode,
            } => {
                match repo {
                    GitRepo::Local(repo) => repo.reset(&commit, reset_mode)?,
                    GitRepo::Remote {
                        project_id,
                        client,
                        worktree_id,
                        work_directory_id,
                    } => {
                        client
                            .request(proto::GitReset {
                                project_id: project_id.0,
                                worktree_id: worktree_id.to_proto(),
                                work_directory_id: work_directory_id.to_proto(),
                                commit: commit.into(),
                                mode: match reset_mode {
                                    ResetMode::Soft => git_reset::ResetMode::Soft.into(),
                                    ResetMode::Mixed => git_reset::ResetMode::Mixed.into(),
                                },
                            })
                            .await?;
                    }
                }
                Ok(())
            }

            Message::CheckoutFiles {
                repo,
                commit,
                paths,
            } => {
                match repo {
                    GitRepo::Local(repo) => repo.checkout_files(&commit, &paths)?,
                    GitRepo::Remote {
                        project_id,
                        client,
                        worktree_id,
                        work_directory_id,
                    } => {
                        client
                            .request(proto::GitCheckoutFiles {
                                project_id: project_id.0,
                                worktree_id: worktree_id.to_proto(),
                                work_directory_id: work_directory_id.to_proto(),
                                commit: commit.into(),
                                paths: paths
                                    .into_iter()
                                    .map(|p| p.to_string_lossy().to_string())
                                    .collect(),
                            })
                            .await?;
                    }
                }
                Ok(())
            }
||||||| parent of 673f7e3e26 (cachix and fontdb)
            Message::Reset {
                repo,
                commit,
                reset_mode,
            } => {
                match repo {
                    GitRepo::Local(repo) => repo.reset(&commit, reset_mode)?,
                    GitRepo::Remote {
                        project_id,
                        client,
                        worktree_id,
                        work_directory_id,
                    } => {
                        client
                            .request(proto::GitReset {
                                project_id: project_id.0,
                                worktree_id: worktree_id.to_proto(),
                                work_directory_id: work_directory_id.to_proto(),
                                commit: commit.into(),
                                mode: match reset_mode {
                                    ResetMode::Soft => git_reset::ResetMode::Soft.into(),
                                    ResetMode::Mixed => git_reset::ResetMode::Mixed.into(),
                                },
                            })
                            .await?;
                    }
                }
                Ok(())
            }
=======
>>>>>>> 673f7e3e26 (cachix and fontdb)
            Message::Unstage(repo, paths) => {
                match repo {
                    GitRepo::Local(repo) => repo.unstage_paths(&paths)?,
                    GitRepo::Remote {
                        project_id,
                        client,
                        worktree_id,
                        work_directory_id,
                    } => {
                        client
                            .request(proto::Unstage {
                                project_id: project_id.0,
                                worktree_id: worktree_id.to_proto(),
                                work_directory_id: work_directory_id.to_proto(),
                                paths: paths
                                    .into_iter()
                                    .map(|repo_path| repo_path.to_proto())
                                    .collect(),
                            })
                            .await
                            .context("sending unstage request")?;
                    }
                }
                Ok(())
            }
            Message::Commit {
                git_repo,
                message,
                name_and_email,
            } => {
                match git_repo {
                    GitRepo::Local(repo) => repo.commit(
                        message.as_ref(),
                        name_and_email
                            .as_ref()
                            .map(|(name, email)| (name.as_ref(), email.as_ref())),
                    )?,
                    GitRepo::Remote {
                        project_id,
                        client,
                        worktree_id,
                        work_directory_id,
                    } => {
                        let (name, email) = name_and_email.unzip();
                        client
                            .request(proto::Commit {
                                project_id: project_id.0,
                                worktree_id: worktree_id.to_proto(),
                                work_directory_id: work_directory_id.to_proto(),
                                message: String::from(message),
                                name: name.map(String::from),
                                email: email.map(String::from),
                            })
                            .await
                            .context("sending commit request")?;
                    }
                }
                Ok(())
            }
        }
    }
<<<<<<< HEAD

    async fn handle_stage(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Stage>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let entries = envelope
            .payload
            .paths
            .into_iter()
            .map(PathBuf::from)
            .map(RepoPath::new)
            .collect();

        repository_handle
            .update(&mut cx, |repository_handle, cx| {
                repository_handle.stage_entries(entries, cx)
            })?
            .await?;
        Ok(proto::Ack {})
    }

    async fn handle_unstage(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Unstage>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let entries = envelope
            .payload
            .paths
            .into_iter()
            .map(PathBuf::from)
            .map(RepoPath::new)
            .collect();

        repository_handle
            .update(&mut cx, |repository_handle, cx| {
                repository_handle.unstage_entries(entries, cx)
            })?
            .await?;

        Ok(proto::Ack {})
    }

    async fn handle_set_index_text(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::SetIndexText>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        repository_handle
            .update(&mut cx, |repository_handle, _| {
                repository_handle.set_index_text(
                    &RepoPath::from_str(&envelope.payload.path),
                    envelope.payload.text,
                )
            })?
            .await??;
        Ok(proto::Ack {})
    }

    async fn handle_commit(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Commit>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let message = SharedString::from(envelope.payload.message);
        let name = envelope.payload.name.map(SharedString::from);
        let email = envelope.payload.email.map(SharedString::from);

        repository_handle
            .update(&mut cx, |repository_handle, _| {
                repository_handle.commit(message, name.zip(email))
            })?
            .await??;
        Ok(proto::Ack {})
    }

    async fn handle_show(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GitShow>,
        mut cx: AsyncApp,
    ) -> Result<proto::GitCommitDetails> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let commit = repository_handle
            .update(&mut cx, |repository_handle, cx| {
                repository_handle.show(&envelope.payload.commit, cx)
            })?
            .await?;
        Ok(proto::GitCommitDetails {
            sha: commit.sha.into(),
            message: commit.message.into(),
            commit_timestamp: commit.commit_timestamp,
            committer_email: commit.committer_email.into(),
            committer_name: commit.committer_name.into(),
        })
    }

    async fn handle_reset(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GitReset>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let mode = match envelope.payload.mode() {
            git_reset::ResetMode::Soft => ResetMode::Soft,
            git_reset::ResetMode::Mixed => ResetMode::Mixed,
        };

        repository_handle
            .update(&mut cx, |repository_handle, _| {
                repository_handle.reset(&envelope.payload.commit, mode)
            })?
            .await??;
        Ok(proto::Ack {})
    }

    async fn handle_checkout_files(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GitCheckoutFiles>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;
        let paths = envelope
            .payload
            .paths
            .iter()
            .map(|s| RepoPath::from_str(s))
            .collect();

        repository_handle
            .update(&mut cx, |repository_handle, _| {
                repository_handle.checkout_files(&envelope.payload.commit, paths)
            })?
            .await??;
        Ok(proto::Ack {})
    }

    async fn handle_open_commit_message_buffer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::OpenCommitMessageBuffer>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferResponse> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;
        let buffer = repository
            .update(&mut cx, |repository, cx| {
                repository.open_commit_buffer(None, this.read(cx).buffer_store.clone(), cx)
            })?
            .await?;

        let buffer_id = buffer.read_with(&cx, |buffer, _| buffer.remote_id())?;
        this.update(&mut cx, |this, cx| {
            this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store
                    .create_buffer_for_peer(
                        &buffer,
                        envelope.original_sender_id.unwrap_or(envelope.sender_id),
                        cx,
                    )
                    .detach_and_log_err(cx);
            })
        })?;

        Ok(proto::OpenBufferResponse {
            buffer_id: buffer_id.to_proto(),
        })
    }

    fn repository_for_request(
        this: &Entity<Self>,
        worktree_id: WorktreeId,
        work_directory_id: ProjectEntryId,
        cx: &mut AsyncApp,
    ) -> Result<Entity<Repository>> {
        this.update(cx, |this, cx| {
            let repository_handle = this
                .all_repositories()
                .into_iter()
                .find(|repository_handle| {
                    repository_handle.read(cx).worktree_id == worktree_id
                        && repository_handle
                            .read(cx)
                            .repository_entry
                            .work_directory_id()
                            == work_directory_id
                })
                .context("missing repository handle")?;
            anyhow::Ok(repository_handle)
        })?
    }
||||||| parent of 673f7e3e26 (cachix and fontdb)

    async fn handle_stage(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Stage>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let entries = envelope
            .payload
            .paths
            .into_iter()
            .map(PathBuf::from)
            .map(RepoPath::new)
            .collect();

        repository_handle
            .update(&mut cx, |repository_handle, cx| {
                repository_handle.stage_entries(entries, cx)
            })?
            .await?;
        Ok(proto::Ack {})
    }

    async fn handle_unstage(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Unstage>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let entries = envelope
            .payload
            .paths
            .into_iter()
            .map(PathBuf::from)
            .map(RepoPath::new)
            .collect();

        repository_handle
            .update(&mut cx, |repository_handle, cx| {
                repository_handle.unstage_entries(entries, cx)
            })?
            .await?;

        Ok(proto::Ack {})
    }

    async fn handle_set_index_text(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::SetIndexText>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        repository_handle
            .update(&mut cx, |repository_handle, _| {
                repository_handle.set_index_text(
                    &RepoPath::from_str(&envelope.payload.path),
                    envelope.payload.text,
                )
            })?
            .await??;
        Ok(proto::Ack {})
    }

    async fn handle_commit(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::Commit>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let message = SharedString::from(envelope.payload.message);
        let name = envelope.payload.name.map(SharedString::from);
        let email = envelope.payload.email.map(SharedString::from);

        repository_handle
            .update(&mut cx, |repository_handle, _| {
                repository_handle.commit(message, name.zip(email))
            })?
            .await??;
        Ok(proto::Ack {})
    }

    async fn handle_show(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GitShow>,
        mut cx: AsyncApp,
    ) -> Result<proto::GitCommitDetails> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let commit = repository_handle
            .update(&mut cx, |repository_handle, cx| {
                repository_handle.show(&envelope.payload.commit, cx)
            })?
            .await?;
        Ok(proto::GitCommitDetails {
            sha: commit.sha.into(),
            message: commit.message.into(),
            commit_timestamp: commit.commit_timestamp,
            committer_email: commit.committer_email.into(),
            committer_name: commit.committer_name.into(),
        })
    }

    async fn handle_reset(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::GitReset>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository_handle =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;

        let mode = match envelope.payload.mode() {
            git_reset::ResetMode::Soft => ResetMode::Soft,
            git_reset::ResetMode::Mixed => ResetMode::Mixed,
        };

        repository_handle
            .update(&mut cx, |repository_handle, _| {
                repository_handle.reset(&envelope.payload.commit, mode)
            })?
            .await??;
        Ok(proto::Ack {})
    }

    async fn handle_open_commit_message_buffer(
        this: Entity<Self>,
        envelope: TypedEnvelope<proto::OpenCommitMessageBuffer>,
        mut cx: AsyncApp,
    ) -> Result<proto::OpenBufferResponse> {
        let worktree_id = WorktreeId::from_proto(envelope.payload.worktree_id);
        let work_directory_id = ProjectEntryId::from_proto(envelope.payload.work_directory_id);
        let repository =
            Self::repository_for_request(&this, worktree_id, work_directory_id, &mut cx)?;
        let buffer = repository
            .update(&mut cx, |repository, cx| {
                repository.open_commit_buffer(None, this.read(cx).buffer_store.clone(), cx)
            })?
            .await?;

        let buffer_id = buffer.read_with(&cx, |buffer, _| buffer.remote_id())?;
        this.update(&mut cx, |this, cx| {
            this.buffer_store.update(cx, |buffer_store, cx| {
                buffer_store
                    .create_buffer_for_peer(
                        &buffer,
                        envelope.original_sender_id.unwrap_or(envelope.sender_id),
                        cx,
                    )
                    .detach_and_log_err(cx);
            })
        })?;

        Ok(proto::OpenBufferResponse {
            buffer_id: buffer_id.to_proto(),
        })
    }

    fn repository_for_request(
        this: &Entity<Self>,
        worktree_id: WorktreeId,
        work_directory_id: ProjectEntryId,
        cx: &mut AsyncApp,
    ) -> Result<Entity<Repository>> {
        this.update(cx, |this, cx| {
            let repository_handle = this
                .all_repositories()
                .into_iter()
                .find(|repository_handle| {
                    repository_handle.read(cx).worktree_id == worktree_id
                        && repository_handle
                            .read(cx)
                            .repository_entry
                            .work_directory_id()
                            == work_directory_id
                })
                .context("missing repository handle")?;
            anyhow::Ok(repository_handle)
        })?
    }
=======
>>>>>>> 673f7e3e26 (cachix and fontdb)
}

impl Repository {
    fn id(&self) -> (WorktreeId, ProjectEntryId) {
        (self.worktree_id, self.repository_entry.work_directory_id())
    }

    pub fn branch(&self) -> Option<Arc<str>> {
        self.repository_entry.branch()
    }

    pub fn display_name(&self, project: &Project, cx: &App) -> SharedString {
        maybe!({
            let project_path = self.repo_path_to_project_path(&"".into())?;
            let worktree_name = project
                .worktree_for_id(project_path.worktree_id, cx)?
                .read(cx)
                .root_name();

            let mut path = PathBuf::new();
            path = path.join(worktree_name);
            path = path.join(project_path.path);
            Some(path.to_string_lossy().to_string())
        })
        .unwrap_or_else(|| self.repository_entry.work_directory.display_name())
        .into()
    }

    pub fn activate(&self, cx: &mut Context<Self>) {
        let Some(git_state) = self.git_state.upgrade() else {
            return;
        };
        let entity = cx.entity();
        git_state.update(cx, |git_state, cx| {
            let Some(index) = git_state
                .repositories
                .iter()
                .position(|handle| *handle == entity)
            else {
                return;
            };
            git_state.active_index = Some(index);
            cx.emit(GitEvent::ActiveRepositoryChanged);
        });
    }

    pub fn status(&self) -> impl '_ + Iterator<Item = StatusEntry> {
        self.repository_entry.status()
    }

    pub fn has_conflict(&self, path: &RepoPath) -> bool {
        self.repository_entry
            .current_merge_conflicts
            .contains(&path)
    }

    pub fn repo_path_to_project_path(&self, path: &RepoPath) -> Option<ProjectPath> {
        let path = self.repository_entry.unrelativize(path)?;
        Some((self.worktree_id, path).into())
    }

    pub fn project_path_to_repo_path(&self, path: &ProjectPath) -> Option<RepoPath> {
        self.worktree_id_path_to_repo_path(path.worktree_id, &path.path)
    }

    pub fn worktree_id_path_to_repo_path(
        &self,
        worktree_id: WorktreeId,
        path: &Path,
    ) -> Option<RepoPath> {
        if worktree_id != self.worktree_id {
            return None;
        }
        self.repository_entry.relativize(path).log_err()
    }

    pub fn open_commit_buffer(
        &mut self,
        languages: Option<Arc<LanguageRegistry>>,
        buffer_store: Entity<BufferStore>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<Entity<Buffer>>> {
        if let Some(buffer) = self.commit_message_buffer.clone() {
            return Task::ready(Ok(buffer));
        }

        if let GitRepo::Remote {
            project_id,
            client,
            worktree_id,
            work_directory_id,
        } = self.git_repo.clone()
        {
            let client = client.clone();
            cx.spawn(|repository, mut cx| async move {
                let request = client.request(proto::OpenCommitMessageBuffer {
                    project_id: project_id.0,
                    worktree_id: worktree_id.to_proto(),
                    work_directory_id: work_directory_id.to_proto(),
                });
                let response = request.await.context("requesting to open commit buffer")?;
                let buffer_id = BufferId::new(response.buffer_id)?;
                let buffer = buffer_store
                    .update(&mut cx, |buffer_store, cx| {
                        buffer_store.wait_for_remote_buffer(buffer_id, cx)
                    })?
                    .await?;
                if let Some(language_registry) = languages {
                    let git_commit_language =
                        language_registry.language_for_name("Git Commit").await?;
                    buffer.update(&mut cx, |buffer, cx| {
                        buffer.set_language(Some(git_commit_language), cx);
                    })?;
                }
                repository.update(&mut cx, |repository, _| {
                    repository.commit_message_buffer = Some(buffer.clone());
                })?;
                Ok(buffer)
            })
        } else {
            self.open_local_commit_buffer(languages, buffer_store, cx)
        }
    }

    fn open_local_commit_buffer(
        &mut self,
        language_registry: Option<Arc<LanguageRegistry>>,
        buffer_store: Entity<BufferStore>,
        cx: &mut Context<Self>,
<<<<<<< HEAD
    ) -> Task<Result<Entity<Buffer>>> {
        let merge_message = self.merge_message.clone();
||||||| parent of 673f7e3e26 (cachix and fontdb)
    ) -> Task<Result<Entity<Buffer>>> {
=======
    ) -> Task<anyhow::Result<Entity<Buffer>>> {
>>>>>>> 673f7e3e26 (cachix and fontdb)
        cx.spawn(|repository, mut cx| async move {
            let buffer = buffer_store
                .update(&mut cx, |buffer_store, cx| buffer_store.create_buffer(cx))?
                .await?;

            if let Some(language_registry) = language_registry {
                let git_commit_language = language_registry.language_for_name("Git Commit").await?;
                buffer.update(&mut cx, |buffer, cx| {
                    buffer.set_language(Some(git_commit_language), cx);
                })?;
            }

            if let Some(merge_message) = merge_message {
                buffer.update(&mut cx, |buffer, cx| {
                    buffer.set_text(merge_message.as_str(), cx)
                })?;
            }

            repository.update(&mut cx, |repository, _| {
                repository.commit_message_buffer = Some(buffer.clone());
            })?;
            Ok(buffer)
        })
    }

<<<<<<< HEAD
    pub fn checkout_files(
        &self,
        commit: &str,
        paths: Vec<RepoPath>,
    ) -> oneshot::Receiver<Result<()>> {
        let (result_tx, result_rx) = futures::channel::oneshot::channel();
        let commit = commit.to_string().into();
        self.update_sender
            .unbounded_send((
                Message::CheckoutFiles {
                    repo: self.git_repo.clone(),
                    commit,
                    paths,
                },
                result_tx,
            ))
            .ok();
        result_rx
    }

    pub fn reset(&self, commit: &str, reset_mode: ResetMode) -> oneshot::Receiver<Result<()>> {
||||||| parent of 673f7e3e26 (cachix and fontdb)
    pub fn reset(&self, commit: &str, reset_mode: ResetMode) -> oneshot::Receiver<Result<()>> {
=======
    pub fn stage_entries(&self, entries: Vec<RepoPath>) -> oneshot::Receiver<anyhow::Result<()>> {
>>>>>>> 673f7e3e26 (cachix and fontdb)
        let (result_tx, result_rx) = futures::channel::oneshot::channel();
        if entries.is_empty() {
            result_tx.send(Ok(())).ok();
            return result_rx;
        }
        self.update_sender
            .unbounded_send((Message::Stage(self.git_repo.clone(), entries), result_tx))
            .ok();
        result_rx
    }

    pub fn unstage_entries(&self, entries: Vec<RepoPath>) -> oneshot::Receiver<anyhow::Result<()>> {
        let (result_tx, result_rx) = futures::channel::oneshot::channel();
        if entries.is_empty() {
            result_tx.send(Ok(())).ok();
            return result_rx;
        }
        self.update_sender
            .unbounded_send((Message::Unstage(self.git_repo.clone(), entries), result_tx))
            .ok();
        result_rx
    }

    pub fn stage_all(&self) -> oneshot::Receiver<anyhow::Result<()>> {
        let to_stage = self
            .repository_entry
            .status()
            .filter(|entry| !entry.status.is_staged().unwrap_or(false))
            .map(|entry| entry.repo_path.clone())
            .collect();
        self.stage_entries(to_stage)
    }

    pub fn unstage_all(&self) -> oneshot::Receiver<anyhow::Result<()>> {
        let to_unstage = self
            .repository_entry
            .status()
            .filter(|entry| entry.status.is_staged().unwrap_or(true))
            .map(|entry| entry.repo_path.clone())
            .collect();
        self.unstage_entries(to_unstage)
    }

    /// Get a count of all entries in the active repository, including
    /// untracked files.
    pub fn entry_count(&self) -> usize {
        self.repository_entry.status_len()
    }

    fn have_changes(&self) -> bool {
        self.repository_entry.status_summary() != GitSummary::UNCHANGED
    }

    fn have_staged_changes(&self) -> bool {
        self.repository_entry.status_summary().index != TrackedSummary::UNCHANGED
    }

    pub fn can_commit(&self, commit_all: bool) -> bool {
        return self.have_changes() && (commit_all || self.have_staged_changes());
    }

    pub fn commit(
        &self,
        message: SharedString,
        name_and_email: Option<(SharedString, SharedString)>,
    ) -> oneshot::Receiver<anyhow::Result<()>> {
        let (result_tx, result_rx) = futures::channel::oneshot::channel();
        self.update_sender
            .unbounded_send((
                Message::Commit {
                    git_repo: self.git_repo.clone(),
                    message,
                    name_and_email,
                },
                result_tx,
            ))
            .ok();
        result_rx
    }
}
