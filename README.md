# RUST + Actix-Web 서버

## 프로젝트 설명

이 프로젝트는 Rust와 actix-web을 이용하여 빠른 성능의 서버를 구축한 프로젝트입니다. 프로젝트의 주요 기능 및 특징은 다음과 같습니다:

1. 이메일을 이용한 로그인 구현

   - 사용자는 이메일 주소와 비밀번호를 입력하여 로그인할 수 있습니다.
   - 이메일 주소의 유효성 검사를 통해 올바른 이메일 주소인지 확인합니다.
   - 비밀번호는 안전하게 암호화되어 저장됩니다.

2. 카카오 로그인 구현

   - 사용자는 카카오 계정을 이용하여 간편하게 로그인할 수 있습니다.
   - 카카오 API를 이용하여 인증 토큰을 발급받습니다.
   - 인증 토큰을 이용하여 사용자의 프로필 정보를 받아와서 로그인 처리를 합니다.

3. diesel을 이용한 postgresql 데이터베이스 관리 및 migration
   - Diesel은 Rust의 ORM (Object-Relational Mapping) 라이브러리로, 데이터베이스와 Rust 코드간의 상호작용을 쉽게 만들어줍니다.
   - Diesel을 이용하여 PostgreSQL 데이터베이스의 스키마 관리와 마이그레이션을 처리합니다.
   - 데이터베이스 연결과 쿼리 작성을 효과적으로 수행합니다.
4. rust를 이용해서 기존 node.js로 작성한 서버보다 더 빠른 성능을 가지고 있습니다.

   - Rust는 시스템 프로그래밍 언어로, 높은 성능을 자랑합니다.
   - 기존 Node.js로 작성된 서버에 비해 더 빠른 성능을 제공합니다.
   - 적은 자원으로 높은 처리량을 달성합니다.

5. 유저 CRUD
   - 사용자 데이터의 생성(C), 조회(R), 수정(U), 삭제(D)를 처리하는 기능을 제공합니다.
   - 사용자 프로필, 비밀번호 변경, 계정 탈퇴 등의 기능을 구현합니다.
   - 사용자 데이터는 PostgreSQL 데이터베이스에 안전하게 저장됩니다.
   - 유저 뿐만 아니라 메세지등의 CRUD도 구현합니다.
6. middleware + jwt를 이용한 세션 관리
   - JSON Web Token (JWT)를 이용하여 사용자 세션을 관리합니다.
   - 로그인 후 발급되는 JWT를 이용하여 사용자 인증을 처리합니다.
   - 미들웨어를 이용하여 모든 API 요청에 대해 사용자 인증을 검사합니다.
7. Web Socket을 이용한 실시간 통신 구현
   - Web Socket 프로토콜을 이용하여 실시간으로 서버와 클라이언트 간의 통신을 가능하게 합니다.
   - 채팅, 알림, 실시간 업데이트 등의 기능을 구현합니다.
   - 서버와 클라이언트 사이의 연결을 유지하면서 빠른 데이터 전송을 제공합니다.
8. Redis Pub/Sub를 이용한 메시지 큐 구현
   - Redis Pub/Sub (Publish/Subscribe) 모델을 이용하여 서버 간의 메시지 전송을 처리합니다.
   - Redis의 Pub/Sub 기능은 메시지 큐와 이벤트 알림 시스템을 구현하는 데 사용됩니다.

## SETUP 방법

1. Rust를 설치합니다.

   `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

2. Docker를 설치하고, PostgreSQL과 Redis 컨테이너를 실행합니다. (선택 사항)

   ```
    docker run --name postgresql -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 -d postgres
    docker run --name redis -p 6379:6379 -d redis
   ```

3. 이 리포지토리를 복제합니다.
   `git clone https://github.com/jinseok9338/voice_server.git`

4. 프로젝트 디렉토리로 이동합니다.
5. `.env` 파일을 생성하고, 다음의 환경 변수를 설정합니다.
   ```
   DATABASE_URL=yourpostgresurlfordiesel
   ACCESS_TOKEN_SECRET=secret
   ACCESS_TOKEN_EXPIRES_IN=3600
   REDIS_URL=yourredisurlfordiesel
   ```
6. 개발 환경에서 핫 리로딩을 위해 `cargo-watch`를 설치합니다.
7. `env RUST_BACKTRACE=1 RUST_LOG=debug cargo watch -x run` 명령어를 실행합니다.
8. 서버가 성공적으로 시작되면, 브라우저에서 http://localhost:8000 로 접속하여 확인할 수 있습니다.
