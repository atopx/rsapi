create database rsapi;

CREATE TABLE sys_user
(
    id          BIGSERIAL PRIMARY KEY,
    username    VARCHAR(32) UNIQUE NOT NULL,
    password    CHAR(32)           NOT NULL,
    name        VARCHAR(64)        NOT NULL,
    avatar      VARCHAR(128)       NOT NULL DEFAULT '',
    phone       VARCHAR(20)        NOT NULL DEFAULT '',
    email       VARCHAR(64)        NOT NULL DEFAULT '',
    description TEXT               NOT NULL DEFAULT '',
    deleted_at  TIMESTAMP          NULL     DEFAULT NULL,
    created_at  TIMESTAMP                   DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP                   DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON TABLE sys_user IS '系统用户表';

COMMENT ON COLUMN sys_user.id IS '主键';
COMMENT ON COLUMN sys_user.username IS '唯一用户名，用于登录';
COMMENT ON COLUMN sys_user.password IS '用户密码哈希，建议使用 bcrypt';
COMMENT ON COLUMN sys_user.name IS '用户昵称/真实姓名';
COMMENT ON COLUMN sys_user.avatar IS '头像 URL，默认空字符串';
COMMENT ON COLUMN sys_user.phone IS '用户手机号，格式不做限制';
COMMENT ON COLUMN sys_user.email IS '邮箱地址';
COMMENT ON COLUMN sys_user.description IS '用户备注信息';
COMMENT ON COLUMN sys_user.deleted_at IS '软删除时间，为 NULL 表示未删除';
COMMENT ON COLUMN sys_user.created_at IS '创建时间';
COMMENT ON COLUMN sys_user.updated_at IS '更新时间';

CREATE INDEX idx_sys_user_deleted_at ON sys_user (deleted_at);
CREATE INDEX idx_sys_user_phone ON sys_user (phone);
CREATE INDEX idx_sys_user_email ON sys_user (email);


CREATE TABLE sys_role
(
    id          BIGSERIAL PRIMARY KEY,
    name        VARCHAR(64) UNIQUE NOT NULL,
    description TEXT               NOT NULL DEFAULT '',
    deleted_at  TIMESTAMP          NULL     DEFAULT NULL,
    created_at  TIMESTAMP                   DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP                   DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON TABLE sys_role IS '系统角色表';

COMMENT ON COLUMN sys_role.id IS '主键';
COMMENT ON COLUMN sys_role.name IS '角色名称，唯一';
COMMENT ON COLUMN sys_role.description IS '角色描述';
COMMENT ON COLUMN sys_role.deleted_at IS '软删除时间，为 NULL 表示未删除';
COMMENT ON COLUMN sys_role.created_at IS '创建时间';
COMMENT ON COLUMN sys_role.updated_at IS '更新时间';

CREATE INDEX idx_sys_role_deleted_at ON sys_role (deleted_at);

CREATE TABLE sys_user_role
(
    user_id     BIGINT NOT NULL,
    role_id     BIGINT NOT NULL,
    assigned_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, role_id)
);

COMMENT ON TABLE sys_user_role IS '用户-角色关联表（多对多）';

COMMENT ON COLUMN sys_user_role.user_id IS '关联的用户ID';
COMMENT ON COLUMN sys_user_role.role_id IS '关联的角色ID';
COMMENT ON COLUMN sys_user_role.assigned_at IS '绑定时间';

CREATE INDEX idx_sys_user_role_user_id ON sys_user_role (user_id);
CREATE INDEX idx_sys_user_role_role_id ON sys_user_role (role_id);

CREATE TABLE sys_permission
(
    id          BIGSERIAL PRIMARY KEY,
    uri         VARCHAR(64) NOT NULL,
    method      VARCHAR(8)  NOT NULL,
    description TEXT        NOT NULL DEFAULT '',
    deleted_at  TIMESTAMP   NULL     DEFAULT NULL,
    created_at  TIMESTAMP            DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP            DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (uri, method)
);

COMMENT ON TABLE sys_permission IS '系统权限表，用于 URI + METHOD 控制';

COMMENT ON COLUMN sys_permission.id IS '主键';
COMMENT ON COLUMN sys_permission.uri IS '资源 URI，如 /api/user';
COMMENT ON COLUMN sys_permission.method IS '请求方法，如 GET/POST/PUT';
COMMENT ON COLUMN sys_permission.description IS '权限描述';
COMMENT ON COLUMN sys_permission.deleted_at IS '软删除时间，为 NULL 表示未删除';
COMMENT ON COLUMN sys_permission.created_at IS '创建时间';
COMMENT ON COLUMN sys_permission.updated_at IS '更新时间';

CREATE INDEX idx_sys_permission_deleted_at ON sys_permission (deleted_at);

CREATE TABLE sys_role_permission
(
    role_id       BIGINT NOT NULL,
    permission_id BIGINT NOT NULL,
    assigned_at   TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (role_id, permission_id)
);

COMMENT ON TABLE sys_role_permission IS '角色-权限关联表（多对多）';

COMMENT ON COLUMN sys_role_permission.role_id IS '关联角色ID';
COMMENT ON COLUMN sys_role_permission.permission_id IS '关联权限ID';
COMMENT ON COLUMN sys_role_permission.assigned_at IS '授权时间';

CREATE INDEX idx_sys_role_permission_role_id ON sys_role_permission (role_id);
CREATE INDEX idx_sys_role_permission_permission_id ON sys_role_permission (permission_id);


CREATE TABLE sys_dict
(
    id          BIGSERIAL PRIMARY KEY,                           -- 主键
    type_code   VARCHAR(64)  NOT NULL,                           -- 字典类型标识，如 'gender'
    type_name   VARCHAR(64)  NOT NULL,                           -- 字典类型名称，如 '性别'
    item_key    VARCHAR(64)  NOT NULL,                           -- 字典项键，如 'male'
    item_value  VARCHAR(128) NOT NULL,                           -- 字典项值，如 '男'
    sort        INT                   DEFAULT 0,                 -- 排序字段（升序）
    disabled    BOOLEAN               DEFAULT FALSE,             -- 是否禁用
    description TEXT         NOT NULL DEFAULT '',                -- 备注
    deleted_at  TIMESTAMP    NULL     DEFAULT NULL,              -- 软删除时间
    created_at  TIMESTAMP             DEFAULT CURRENT_TIMESTAMP, -- 创建时间
    updated_at  TIMESTAMP             DEFAULT CURRENT_TIMESTAMP  -- 更新时间
);

CREATE UNIQUE INDEX idx_dict_type_key ON sys_dict (type_code, item_key);
CREATE INDEX idx_dict_type_code ON sys_dict (type_code);
CREATE INDEX idx_dict_deleted_at ON sys_dict (deleted_at);

COMMENT ON TABLE sys_dict IS '系统通用字典表';
COMMENT ON COLUMN sys_dict.type_code IS '字典类型标识，如 gender、status';
COMMENT ON COLUMN sys_dict.type_name IS '字典类型中文名，如 性别、状态';
COMMENT ON COLUMN sys_dict.item_key IS '字典项键名，如 male、active';
COMMENT ON COLUMN sys_dict.item_value IS '字典项值，如 男、启用';
COMMENT ON COLUMN sys_dict.sort IS '排序值，升序排列';
COMMENT ON COLUMN sys_dict.disabled IS '是否启用，true 表示启用';
COMMENT ON COLUMN sys_dict.description IS '描述信息，可用于备注';
COMMENT ON COLUMN sys_dict.deleted_at IS '软删除字段，为 NULL 表示未删除';
COMMENT ON COLUMN sys_dict.created_at IS '创建时间';
COMMENT ON COLUMN sys_dict.updated_at IS '更新时间';
