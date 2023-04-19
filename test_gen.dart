todo remove me for testing
todo remove me for testing
todo remove me for testing
todo remove me for testing
todo remove me for testing
// DO NOT MODIFY THIS FILE
// This file contains automatically generated Dart Bindings.
// It was generated via the clang2src project, and ultimately comes from a set of annotated Rust source files
// Any modifications you make to this file will be reverted whenever this file is regenerated

import 'package:ffi/ffi.dart';

import 'dart:ffi' as ffi;

import 'dart:io' show Platform, Directory;

import 'package:path/path.dart' as path;

class liboauthtoolException implements Exception {
  final String msg;
  final int code;

  const liboauthtoolException(this.msg, this.code);
}

/* Region: C Constants */
const int C_NULL = 0;
const int C_TRUE = 1;
const int C_FALSE = 0;

/* Region: Dart Constants */

const int OAUTHTOOL_PASS = 0;

const int OAUTHTOOL_FAIL = 1;

const int OAUTHTOOL_FAIL_NULL_POINTER = 2;

/* Region: FFI Enums */

/// Code Challenge for PKCE-enabled Authorization Code Flow
/// https://www.rfc-editor.org/rfc/rfc7636#section-4.2
/// If PKCE is not enabled, use [CodeChallengeMethod.None]

enum CodeChallengeMethod {
  /// None, for when the code flow isnt PKCE enabled

  None(0),

  /// Plain: code_challenge=code_verifier

  Plain(1),

  /// Sha256: code_challenge = BASE64URL-ENCODE(SHA256(ASCII(code_verifier)))

  S256(2),
  ;

  final int value;

  const CodeChallengeMethod(this.value);
}

/// Specifies the different types of OAuth Flows
/// Implicit,
/// Client Credentials,
/// Authorization,
/// Authorization + PKCE,
/// Device

enum FlowType {
  /// Implicit Grant

  Implicit(0),

  /// Client Credentials Grant
  /// Gives keys to an application without accessing user-specific resources

  ClientCredentials(1),

  /// Authorization Grant
  /// Allows access to user-specific resources. Usually comes with an infinite-lifespan Refresh Token

  Authorization(2),

  /// Secure Authorization grant using the PKCE extension
  /// Refresh Tokens are one-time-use

  AuthorizationPKCE(3),

  /// Flow for using a second device, which has a screen, to give access to a primary device, which does not have a screen

  Device(4),
  ;

  final int value;

  const FlowType(this.value);
}

/* Region: FFI Structs */

class C_Engine extends ffi.Opaque {}

class C_OAuth2Authorization extends ffi.Opaque {}

class C_OAuth2ClientCredentials extends ffi.Opaque {}

class C_OAuth2Implicit extends ffi.Opaque {}

class C_OAuth2PKCE extends ffi.Opaque {}

class C_FFIArray extends ffi.Struct {
  /// Number of elements in the returned array

  @ffi.UintPtr()
  external int len;

  /// Max size of the array

  @ffi.UintPtr()
  external int cap;

  /// pointer to the first item in the array

  external ffi.Pointer<ffi.Pointer<ffi.Char>> arr;
}

class C_TokenResponse extends ffi.Struct {
  /// If not null, contains a token that can be used to access the service

  external ffi.Pointer<ffi.Char> accessToken;

  /// If not null, contains a token that can be used to get a new access token

  external ffi.Pointer<ffi.Char> refreshToken;

  /// Seconds from received time that the token expires at

  @ffi.Int64()
  external int expiresAt;

  /// If not null, denotes what kind of token this is.  Usually Bearer

  external ffi.Pointer<ffi.Char> tokenType;

  external ffi.Pointer<C_FFIArray> scopes;
}

class C_AuthUrlOutput extends ffi.Struct {
  external ffi.Pointer<ffi.Char> url;

  external ffi.Pointer<ffi.Char> localState;

  external ffi.Pointer<ffi.Char> pkceVerifierState;
}

class C_ParsedAuthorizationCode extends ffi.Struct {
  /// Authorization Code. Always present.

  external ffi.Pointer<ffi.Char> code;

  /// State returned from server. Should match state given to server. Not always present

  external ffi.Pointer<ffi.Char> state;
}

/* Region: FFI Free Functions */

int ffi_encrypt(
  ffi.Pointer<ffi.Char> plain_text,
  ffi.Pointer<ffi.Pointer<ffi.Char>> encrypted_output,
  ffi.Pointer<C_Engine> engine,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_encrypt(
    plain_text,
    encrypted_output,
    engine,
    err_ptr,
  );
}

final ffi_encryptPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
  ffi.Pointer<C_Engine>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('encrypt');
final _ffi_encrypt = ffi_encryptPtr.asFunction<
    int Function(
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
  ffi.Pointer<C_Engine>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

int ffi_decrypt(
  ffi.Pointer<ffi.Char> encrypted_text,
  ffi.Pointer<ffi.Pointer<ffi.Char>> decrypted_output,
  ffi.Pointer<C_Engine> engine,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_decrypt(
    encrypted_text,
    decrypted_output,
    engine,
    err_ptr,
  );
}

final ffi_decryptPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
  ffi.Pointer<C_Engine>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('decrypt');
final _ffi_decrypt = ffi_decryptPtr.asFunction<
    int Function(
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
  ffi.Pointer<C_Engine>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

int ffi_OAuth2PKCE_new(
  ffi.Pointer<ffi.Pointer<C_OAuth2PKCE>> this_,
  ffi.Pointer<ffi.Char> client_id,
  ffi.Pointer<ffi.Char> client_secret,
  ffi.Pointer<ffi.Char> authorization_url,
  ffi.Pointer<ffi.Char> token_url,
  ffi.Pointer<ffi.Char> redirect_url,
  CodeChallengeMethod challenge_method,
  ffi.Pointer<ffi.Char> scopes,
  ffi.Pointer<ffi.Char> extra_parameters,
  int timeout_in_milliseconds,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2PKCE_new(
    this_,
    client_id,
    client_secret,
    authorization_url,
    token_url,
    redirect_url,
    challenge_method.value,
    scopes,
    extra_parameters,
    timeout_in_milliseconds,
    err_ptr,
  );
}

final ffi_OAuth2PKCE_newPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<ffi.Pointer<C_OAuth2PKCE>>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Int32,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Int64,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2PKCE_new');
final _ffi_OAuth2PKCE_new = ffi_OAuth2PKCE_newPtr.asFunction<
    int Function(
  ffi.Pointer<ffi.Pointer<C_OAuth2PKCE>>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  int,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  int,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

int ffi_OAuth2PKCE_get_token_automatic(
  ffi.Pointer<C_OAuth2PKCE> this_,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>> token_output,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2PKCE_get_token_automatic(
    this_,
    token_output,
    err_ptr,
  );
}

final ffi_OAuth2PKCE_get_token_automaticPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2PKCE_get_token_automatic');
final _ffi_OAuth2PKCE_get_token_automatic =
    ffi_OAuth2PKCE_get_token_automaticPtr.asFunction<
        int Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

int ffi_OAuth2PKCE_start_web_server_for_callback(
  ffi.Pointer<C_OAuth2PKCE> this_,
  ffi.Pointer<ffi.Char> redirect_url,
  ffi.Pointer<ffi.Char> verifier_state,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>> token_output,
  int timeout_in_milliseconds,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2PKCE_start_web_server_for_callback(
    this_,
    redirect_url,
    verifier_state,
    token_output,
    timeout_in_milliseconds,
    err_ptr,
  );
}

final ffi_OAuth2PKCE_start_web_server_for_callbackPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Int64,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2PKCE_start_web_server_for_callback');
final _ffi_OAuth2PKCE_start_web_server_for_callback =
    ffi_OAuth2PKCE_start_web_server_for_callbackPtr.asFunction<
        int Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  int,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

int ffi_OAuth2PKCE_get_authorization_url(
  ffi.Pointer<C_OAuth2PKCE> this_,
  ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>> authorization_output,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2PKCE_get_authorization_url(
    this_,
    authorization_output,
    err_ptr,
  );
}

final ffi_OAuth2PKCE_get_authorization_urlPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2PKCE_get_authorization_url');
final _ffi_OAuth2PKCE_get_authorization_url =
    ffi_OAuth2PKCE_get_authorization_urlPtr.asFunction<
        int Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

int ffi_OAuth2PKCE_exchange_authorization_code_for_token(
  ffi.Pointer<C_OAuth2PKCE> this_,
  ffi.Pointer<ffi.Char> authorization_code,
  ffi.Pointer<ffi.Char> verifier_state,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>> token_output,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2PKCE_exchange_authorization_code_for_token(
    this_,
    authorization_code,
    verifier_state,
    token_output,
    err_ptr,
  );
}

final ffi_OAuth2PKCE_exchange_authorization_code_for_tokenPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2PKCE_exchange_authorization_code_for_token');
final _ffi_OAuth2PKCE_exchange_authorization_code_for_token =
    ffi_OAuth2PKCE_exchange_authorization_code_for_tokenPtr.asFunction<
        int Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

int ffi_OAuth2PKCE_refresh_access_token(
  ffi.Pointer<C_OAuth2PKCE> this_,
  ffi.Pointer<ffi.Char> refresh_token,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>> token_output,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2PKCE_refresh_access_token(
    this_,
    refresh_token,
    token_output,
    err_ptr,
  );
}

final ffi_OAuth2PKCE_refresh_access_tokenPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2PKCE_refresh_access_token');
final _ffi_OAuth2PKCE_refresh_access_token =
    ffi_OAuth2PKCE_refresh_access_tokenPtr.asFunction<
        int Function(
  ffi.Pointer<C_OAuth2PKCE>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

int ffi_OAuth2Authorization_new(
  ffi.Pointer<ffi.Pointer<C_OAuth2Authorization>> this_,
  ffi.Pointer<ffi.Char> client_id,
  ffi.Pointer<ffi.Char> client_secret,
  ffi.Pointer<ffi.Char> authorization_url,
  ffi.Pointer<ffi.Char> token_url,
  ffi.Pointer<ffi.Char> redirect_url,
  ffi.Pointer<ffi.Char> scopes,
  ffi.Pointer<ffi.Char> extra_parameters,
  int timeout_in_milliseconds,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2Authorization_new(
    this_,
    client_id,
    client_secret,
    authorization_url,
    token_url,
    redirect_url,
    scopes,
    extra_parameters,
    timeout_in_milliseconds,
    err_ptr,
  );
}

final ffi_OAuth2Authorization_newPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<ffi.Pointer<C_OAuth2Authorization>>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Int64,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2Authorization_new');
final _ffi_OAuth2Authorization_new = ffi_OAuth2Authorization_newPtr.asFunction<
    int Function(
  ffi.Pointer<ffi.Pointer<C_OAuth2Authorization>>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Char>,
  int,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

int ffi_OAuth2Authorization_get_authorization_url(
  ffi.Pointer<C_OAuth2Authorization> this_,
  ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>> authorization_output,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2Authorization_get_authorization_url(
    this_,
    authorization_output,
    err_ptr,
  );
}

final ffi_OAuth2Authorization_get_authorization_urlPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<C_OAuth2Authorization>,
  ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2Authorization_get_authorization_url');
final _ffi_OAuth2Authorization_get_authorization_url =
    ffi_OAuth2Authorization_get_authorization_urlPtr.asFunction<
        int Function(
  ffi.Pointer<C_OAuth2Authorization>,
  ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

int ffi_OAuth2Authorization_exchange_authorization_code_for_token(
  ffi.Pointer<C_OAuth2Authorization> this_,
  ffi.Pointer<ffi.Char> authorization_code,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>> token_output,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2Authorization_exchange_authorization_code_for_token(
    this_,
    authorization_code,
    token_output,
    err_ptr,
  );
}

final ffi_OAuth2Authorization_exchange_authorization_code_for_tokenPtr =
    _lookup<
        ffi.NativeFunction<
            ffi.Uint32 Function(
  ffi.Pointer<C_OAuth2Authorization>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2Authorization_exchange_authorization_code_for_token');
final _ffi_OAuth2Authorization_exchange_authorization_code_for_token =
    ffi_OAuth2Authorization_exchange_authorization_code_for_tokenPtr.asFunction<
        int Function(
  ffi.Pointer<C_OAuth2Authorization>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

int ffi_OAuth2Authorization_refresh_access_token(
  ffi.Pointer<C_OAuth2Authorization> this_,
  ffi.Pointer<ffi.Char> refresh_token,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>> token_output,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_OAuth2Authorization_refresh_access_token(
    this_,
    refresh_token,
    token_output,
    err_ptr,
  );
}

final ffi_OAuth2Authorization_refresh_access_tokenPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<C_OAuth2Authorization>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('OAuth2Authorization_refresh_access_token');
final _ffi_OAuth2Authorization_refresh_access_token =
    ffi_OAuth2Authorization_refresh_access_tokenPtr.asFunction<
        int Function(
  ffi.Pointer<C_OAuth2Authorization>,
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

int ffi_parse_authorization_callback_url(
  ffi.Pointer<ffi.Char> filled_callback_url,
  ffi.Pointer<ffi.Pointer<C_ParsedAuthorizationCode>>
      parsed_authorization_code_output,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_parse_authorization_callback_url(
    filled_callback_url,
    parsed_authorization_code_output,
    err_ptr,
  );
}

final ffi_parse_authorization_callback_urlPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_ParsedAuthorizationCode>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('parse_authorization_callback_url');
final _ffi_parse_authorization_callback_url =
    ffi_parse_authorization_callback_urlPtr.asFunction<
        int Function(
  ffi.Pointer<ffi.Char>,
  ffi.Pointer<ffi.Pointer<C_ParsedAuthorizationCode>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

int ffi_Engine_new(
  ffi.Pointer<ffi.Pointer<C_Engine>> engine,
  ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
) {
  return _ffi_Engine_new(
    engine,
    err_ptr,
  );
}

final ffi_Engine_newPtr = _lookup<
    ffi.NativeFunction<
        ffi.Uint32 Function(
  ffi.Pointer<ffi.Pointer<C_Engine>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>>('Engine_new');
final _ffi_Engine_new = ffi_Engine_newPtr.asFunction<
    int Function(
  ffi.Pointer<ffi.Pointer<C_Engine>>,
  ffi.Pointer<ffi.Pointer<ffi.Char>>,
)>();

void ffi_Engine_free(
  ffi.Pointer<C_Engine> engine,
) {
  return _ffi_Engine_free(
    engine,
  );
}

final ffi_Engine_freePtr = _lookup<
    ffi.NativeFunction<
        ffi.Void Function(
  ffi.Pointer<C_Engine>,
)>>('Engine_free');
final _ffi_Engine_free = ffi_Engine_freePtr.asFunction<
    void Function(
  ffi.Pointer<C_Engine>,
)>();

void ffi_OAuth2PKCE_free(
  ffi.Pointer<C_OAuth2PKCE> mgr,
) {
  return _ffi_OAuth2PKCE_free(
    mgr,
  );
}

final ffi_OAuth2PKCE_freePtr = _lookup<
    ffi.NativeFunction<
        ffi.Void Function(
  ffi.Pointer<C_OAuth2PKCE>,
)>>('OAuth2PKCE_free');
final _ffi_OAuth2PKCE_free = ffi_OAuth2PKCE_freePtr.asFunction<
    void Function(
  ffi.Pointer<C_OAuth2PKCE>,
)>();

void ffi_OAuth2Authorization_free(
  ffi.Pointer<C_OAuth2Authorization> mgr,
) {
  return _ffi_OAuth2Authorization_free(
    mgr,
  );
}

final ffi_OAuth2Authorization_freePtr = _lookup<
    ffi.NativeFunction<
        ffi.Void Function(
  ffi.Pointer<C_OAuth2Authorization>,
)>>('OAuth2Authorization_free');
final _ffi_OAuth2Authorization_free =
    ffi_OAuth2Authorization_freePtr.asFunction<
        void Function(
  ffi.Pointer<C_OAuth2Authorization>,
)>();

void ffi_OAuth2ClientCredentials_free(
  ffi.Pointer<C_OAuth2ClientCredentials> mgr,
) {
  return _ffi_OAuth2ClientCredentials_free(
    mgr,
  );
}

final ffi_OAuth2ClientCredentials_freePtr = _lookup<
    ffi.NativeFunction<
        ffi.Void Function(
  ffi.Pointer<C_OAuth2ClientCredentials>,
)>>('OAuth2ClientCredentials_free');
final _ffi_OAuth2ClientCredentials_free =
    ffi_OAuth2ClientCredentials_freePtr.asFunction<
        void Function(
  ffi.Pointer<C_OAuth2ClientCredentials>,
)>();

void ffi_OAuth2Implicit_free(
  ffi.Pointer<C_OAuth2Implicit> mgr,
) {
  return _ffi_OAuth2Implicit_free(
    mgr,
  );
}

final ffi_OAuth2Implicit_freePtr = _lookup<
    ffi.NativeFunction<
        ffi.Void Function(
  ffi.Pointer<C_OAuth2Implicit>,
)>>('OAuth2Implicit_free');
final _ffi_OAuth2Implicit_free = ffi_OAuth2Implicit_freePtr.asFunction<
    void Function(
  ffi.Pointer<C_OAuth2Implicit>,
)>();

ffi.Pointer<C_FFIArray> ffi_TestGetRustStringList() {
  return _ffi_TestGetRustStringList();
}

final ffi_TestGetRustStringListPtr =
    _lookup<ffi.NativeFunction<ffi.Pointer<C_FFIArray> Function()>>(
        'TestGetRustStringList');
final _ffi_TestGetRustStringList = ffi_TestGetRustStringListPtr
    .asFunction<ffi.Pointer<C_FFIArray> Function()>();

/* Region: Dart Classes for use by the end-user */

class Engine implements IWithPtr, ffi.Finalizable {
  /* Fields */

  ffi.Pointer<C_Engine> _selfPtr;

  /// Pointer to the backing `free` function which disposes the backing pointer when this dart object is GC'd

  static final ffi.NativeFinalizer _finalizer =
      ffi.NativeFinalizer(ffi_Engine_freePtr.cast());

  /* Constructors */
  Engine._(this._selfPtr) {
    _finalizer.attach(this, _selfPtr.cast(), detach: this);
  }

  /* Functions */

  factory Engine._fromCPointerPointer(
    ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr,
  ) {
    ffi.Pointer<ffi.Pointer<C_Engine>> ptr = voidPtr.cast();
    final _innerPtr = ptr.value;
    return Engine._(_innerPtr);
  }

  @override
  ffi.Pointer<ffi.Void> getPointer() {
    return _selfPtr.cast();
  }
}

class OAuth2Authorization implements IWithPtr, ffi.Finalizable {
  /* Fields */

  ffi.Pointer<C_OAuth2Authorization> _selfPtr;

  /// Pointer to the backing `free` function which disposes the backing pointer when this dart object is GC'd

  static final ffi.NativeFinalizer _finalizer =
      ffi.NativeFinalizer(ffi_OAuth2Authorization_freePtr.cast());

  /* Constructors */
  OAuth2Authorization._(this._selfPtr) {
    _finalizer.attach(this, _selfPtr.cast(), detach: this);
  }

  /* Functions */

  factory OAuth2Authorization._fromCPointerPointer(
    ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr,
  ) {
    ffi.Pointer<ffi.Pointer<C_OAuth2Authorization>> ptr = voidPtr.cast();
    final _innerPtr = ptr.value;
    return OAuth2Authorization._(_innerPtr);
  }

  @override
  ffi.Pointer<ffi.Void> getPointer() {
    return _selfPtr.cast();
  }
}

class OAuth2ClientCredentials implements IWithPtr, ffi.Finalizable {
  /* Fields */

  ffi.Pointer<C_OAuth2ClientCredentials> _selfPtr;

  /// Pointer to the backing `free` function which disposes the backing pointer when this dart object is GC'd

  static final ffi.NativeFinalizer _finalizer =
      ffi.NativeFinalizer(ffi_OAuth2ClientCredentials_freePtr.cast());

  /* Constructors */
  OAuth2ClientCredentials._(this._selfPtr) {
    _finalizer.attach(this, _selfPtr.cast(), detach: this);
  }

  /* Functions */

  factory OAuth2ClientCredentials._fromCPointerPointer(
    ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr,
  ) {
    ffi.Pointer<ffi.Pointer<C_OAuth2ClientCredentials>> ptr = voidPtr.cast();
    final _innerPtr = ptr.value;
    return OAuth2ClientCredentials._(_innerPtr);
  }

  @override
  ffi.Pointer<ffi.Void> getPointer() {
    return _selfPtr.cast();
  }
}

class OAuth2Implicit implements IWithPtr, ffi.Finalizable {
  /* Fields */

  ffi.Pointer<C_OAuth2Implicit> _selfPtr;

  /// Pointer to the backing `free` function which disposes the backing pointer when this dart object is GC'd

  static final ffi.NativeFinalizer _finalizer =
      ffi.NativeFinalizer(ffi_OAuth2Implicit_freePtr.cast());

  /* Constructors */
  OAuth2Implicit._(this._selfPtr) {
    _finalizer.attach(this, _selfPtr.cast(), detach: this);
  }

  /* Functions */

  factory OAuth2Implicit._fromCPointerPointer(
    ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr,
  ) {
    ffi.Pointer<ffi.Pointer<C_OAuth2Implicit>> ptr = voidPtr.cast();
    final _innerPtr = ptr.value;
    return OAuth2Implicit._(_innerPtr);
  }

  @override
  ffi.Pointer<ffi.Void> getPointer() {
    return _selfPtr.cast();
  }
}

class OAuth2PKCE implements IWithPtr, ffi.Finalizable {
  /* Fields */

  ffi.Pointer<C_OAuth2PKCE> _selfPtr;

  /// Pointer to the backing `free` function which disposes the backing pointer when this dart object is GC'd

  static final ffi.NativeFinalizer _finalizer =
      ffi.NativeFinalizer(ffi_OAuth2PKCE_freePtr.cast());

  /* Constructors */
  OAuth2PKCE._(this._selfPtr) {
    _finalizer.attach(this, _selfPtr.cast(), detach: this);
  }

  /* Functions */

  factory OAuth2PKCE._fromCPointerPointer(
    ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr,
  ) {
    ffi.Pointer<ffi.Pointer<C_OAuth2PKCE>> ptr = voidPtr.cast();
    final _innerPtr = ptr.value;
    return OAuth2PKCE._(_innerPtr);
  }

  @override
  ffi.Pointer<ffi.Void> getPointer() {
    return _selfPtr.cast();
  }
}

class FFIArray {
  /* Fields */

  /// Number of elements in the returned array

  final int len;

  /// Max size of the array

  final int cap;

  /// pointer to the first item in the array

  final ffi.Pointer<ffi.Pointer<ffi.Char>> arr;

  /* Functions */

  factory FFIArray._fromCPointerPointer(
    ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr,
  ) {
    ffi.Pointer<ffi.Pointer<C_FFIArray>> ptr = voidPtr.cast();
    final st = ptr.value.ref;
    return FFIArray._fromCStruct(st);
  }

  factory FFIArray._fromCStruct(
    C_FFIArray c,
  ) {
    'TODO: ayy lmao is struct and has fields';
  }
}

class TokenResponse {
  /* Fields */

  /// If not null, contains a token that can be used to access the service

  final ffi.Pointer<ffi.Char> accessToken;

  /// If not null, contains a token that can be used to get a new access token

  final ffi.Pointer<ffi.Char>? refreshToken;

  /// Seconds from received time that the token expires at

  final int expiresAt;

  /// If not null, denotes what kind of token this is.  Usually Bearer

  final ffi.Pointer<ffi.Char>? tokenType;

  final ffi.Pointer<C_FFIArray> scopes;

  /* Functions */

  factory TokenResponse._fromCPointerPointer(
    ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr,
  ) {
    ffi.Pointer<ffi.Pointer<C_TokenResponse>> ptr = voidPtr.cast();
    final st = ptr.value.ref;
    return TokenResponse._fromCStruct(st);
  }

  factory TokenResponse._fromCStruct(
    C_TokenResponse c,
  ) {
    'TODO: ayy lmao is struct and has fields';
  }
}

class AuthUrlOutput {
  /* Fields */

  final ffi.Pointer<ffi.Char> url;

  final ffi.Pointer<ffi.Char>? localState;

  final ffi.Pointer<ffi.Char>? pkceVerifierState;

  /* Functions */

  factory AuthUrlOutput._fromCPointerPointer(
    ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr,
  ) {
    ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>> ptr = voidPtr.cast();
    final st = ptr.value.ref;
    return AuthUrlOutput._fromCStruct(st);
  }

  factory AuthUrlOutput._fromCStruct(
    C_AuthUrlOutput c,
  ) {
    'TODO: ayy lmao is struct and has fields';
  }
}

class ParsedAuthorizationCode {
  /* Fields */

  /// Authorization Code. Always present.

  final ffi.Pointer<ffi.Char> code;

  /// State returned from server. Should match state given to server. Not always present

  final ffi.Pointer<ffi.Char>? state;

  /* Functions */

  factory ParsedAuthorizationCode._fromCPointerPointer(
    ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr,
  ) {
    ffi.Pointer<ffi.Pointer<C_ParsedAuthorizationCode>> ptr = voidPtr.cast();
    final st = ptr.value.ref;
    return ParsedAuthorizationCode._fromCStruct(st);
  }

  factory ParsedAuthorizationCode._fromCStruct(
    C_ParsedAuthorizationCode c,
  ) {
    'TODO: ayy lmao is struct and has fields';
  }
}

/* Region: Dart Free Functions */

String encrypt(
  String plain_text,
  Engine engine,
) {
  /* Get error pointer in case function returns failure */
  final ffi.Pointer<ffi.Pointer<ffi.Char>> cErrPtr =
      _getPointerForType<String>().cast();

  /* get pointer types for items that require it*/

  final cplain_textPtr = _getPointerForData(plain_text);

  final cenginePtr = _getPointerForData(engine);

  /* get Output Pointer type */
  final cencryptOutputPtr = _getPointerForType<String>();

  /* call native function */
  int errCode = ffi_encrypt(
    cencryptOutputPtr.cast(),
    cplain_textPtr.cast(),
    cenginePtr.cast(),
    cErrPtr,
  );

  /* Check error code */
  if (errCode != C_FALSE) {
    /* free pointers if required  */

    calloc.free(cplain_textPtr);

    calloc.free(cenginePtr);

    calloc.free(cencryptOutputPtr);

    /* throw final Exception */
    throw liboauthtoolException(_getDartStringFromDoublePtr(cErrPtr), errCode);
  }

  /* Free allocated pointers */

  calloc.free(cplain_textPtr);

  calloc.free(cenginePtr);

  calloc.free(cErrPtr);

  /* return final value */
}

String decrypt(
  String encrypted_text,
  Engine engine,
) {
  /* Get error pointer in case function returns failure */
  final ffi.Pointer<ffi.Pointer<ffi.Char>> cErrPtr =
      _getPointerForType<String>().cast();

  /* get pointer types for items that require it*/

  final cencrypted_textPtr = _getPointerForData(encrypted_text);

  final cenginePtr = _getPointerForData(engine);

  /* get Output Pointer type */
  final cdecryptOutputPtr = _getPointerForType<String>();

  /* call native function */
  int errCode = ffi_decrypt(
    cdecryptOutputPtr.cast(),
    cencrypted_textPtr.cast(),
    cenginePtr.cast(),
    cErrPtr,
  );

  /* Check error code */
  if (errCode != C_FALSE) {
    /* free pointers if required  */

    calloc.free(cencrypted_textPtr);

    calloc.free(cenginePtr);

    calloc.free(cdecryptOutputPtr);

    /* throw final Exception */
    throw liboauthtoolException(_getDartStringFromDoublePtr(cErrPtr), errCode);
  }

  /* Free allocated pointers */

  calloc.free(cencrypted_textPtr);

  calloc.free(cenginePtr);

  calloc.free(cErrPtr);

  /* return final value */
}

ParsedAuthorizationCode parse_authorization_callback_url(
  String filled_callback_url,
) {
  /* Get error pointer in case function returns failure */
  final ffi.Pointer<ffi.Pointer<ffi.Char>> cErrPtr =
      _getPointerForType<String>().cast();

  /* get pointer types for items that require it*/

  final cfilled_callback_urlPtr = _getPointerForData(filled_callback_url);

  /* get Output Pointer type */
  final cparse_authorization_callback_urlOutputPtr =
      _getPointerForType<ParsedAuthorizationCode>();

  /* call native function */
  int errCode = ffi_parse_authorization_callback_url(
    cparse_authorization_callback_urlOutputPtr.cast(),
    cfilled_callback_urlPtr.cast(),
    cErrPtr,
  );

  /* Check error code */
  if (errCode != C_FALSE) {
    /* free pointers if required  */

    calloc.free(cfilled_callback_urlPtr);

    calloc.free(cparse_authorization_callback_urlOutputPtr);

    /* throw final Exception */
    throw liboauthtoolException(_getDartStringFromDoublePtr(cErrPtr), errCode);
  }

  /* Free allocated pointers */

  calloc.free(cfilled_callback_urlPtr);

  calloc.free(cErrPtr);

  /* return final value */
}

ffi.Pointer<C_FFIArray> TestGetRustStringList() {
  'lol?';
}

/* Region: Dart Pointer Utility Functions  */

/// Interface to get a Pointer to the backing data on classes that
/// cross FFI boundaries
abstract class IWithPtr {
  ffi.Pointer<ffi.Void> getPointer();
}

/// Holds the symbol lookup function.
final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
    _lookup = loadLibrary(getLibraryPath()).lookup;

T transformFromPointer<T, E extends ffi.NativeType>(
    ffi.Pointer<ffi.Pointer<E>> data) {
  if (T == String) {
    return _getDartStringFromDoublePtr(data.cast()) as T;
  } else if (T == C_Engine) {
    return Engine._fromCPointerPointer(data.cast()) as T;
  } else if (T == C_OAuth2Authorization) {
    return OAuth2Authorization._fromCPointerPointer(data.cast()) as T;
  } else if (T == C_OAuth2ClientCredentials) {
    return OAuth2ClientCredentials._fromCPointerPointer(data.cast()) as T;
  } else if (T == C_OAuth2Implicit) {
    return OAuth2Implicit._fromCPointerPointer(data.cast()) as T;
  } else if (T == C_OAuth2PKCE) {
    return OAuth2PKCE._fromCPointerPointer(data.cast()) as T;
  } else if (T == C_FFIArray) {
    return FFIArray._fromCPointerPointer(data.cast()) as T;
  } else if (T == C_TokenResponse) {
    return TokenResponse._fromCPointerPointer(data.cast()) as T;
  } else if (T == C_AuthUrlOutput) {
    return AuthUrlOutput._fromCPointerPointer(data.cast()) as T;
  } else if (T == C_ParsedAuthorizationCode) {
    return ParsedAuthorizationCode._fromCPointerPointer(data.cast()) as T;
  }

  throw liboauthtoolException('Invalid data in transformFromPointer: $T', -4);
}

ffi.Pointer<ffi.Void> _getPointerForType<T>() {
  if (T == String) {
    return _getEmptyStringPointer().cast();
  } else if (T == Engine) {
    return calloc<ffi.Pointer<C_Engine>>().cast();
  } else if (T == OAuth2Authorization) {
    return calloc<ffi.Pointer<C_OAuth2Authorization>>().cast();
  } else if (T == OAuth2ClientCredentials) {
    return calloc<ffi.Pointer<C_OAuth2ClientCredentials>>().cast();
  } else if (T == OAuth2Implicit) {
    return calloc<ffi.Pointer<C_OAuth2Implicit>>().cast();
  } else if (T == OAuth2PKCE) {
    return calloc<ffi.Pointer<C_OAuth2PKCE>>().cast();
  } else if (T == FFIArray) {
    return calloc<ffi.Pointer<C_FFIArray>>().cast();
  } else if (T == TokenResponse) {
    return calloc<ffi.Pointer<C_TokenResponse>>().cast();
  } else if (T == AuthUrlOutput) {
    return calloc<ffi.Pointer<C_AuthUrlOutput>>().cast();
  } else if (T == ParsedAuthorizationCode) {
    return calloc<ffi.Pointer<C_ParsedAuthorizationCode>>().cast();
  } else {
    throw liboauthtoolException('Invalid type: $T', -3);
  }
}

/// Returns a castable pointer based on the input data.
/// This function is only valid for Types [String, {custom C generated classes}]
/// Will throw an Exception if passed invalid types
ffi.Pointer<ffi.Void> _getPointerForData(dynamic data) {
  if (data is String) {
    return _stringToFFIPointer(data).cast();
  } else if (data is IWithPtr) {
    return data.getPointer().cast();
  } else {
    throw liboauthtoolException('Invalid data type for pointer: $data', -2);
  }
}

/// Returns a Dart String from a `char*`
///
/// n.b., THIS CONSUMES AND FREES THE POINTER
/// Do not use `charPtr` after this
///
/// For double pointers `char**` use `_getDartStringFromDoublePtr`
String _getDartStringFromPtr(ffi.Pointer<ffi.Char> charPtr) {
  final asUtf8Ptr = charPtr.cast<Utf8>();
  final asDartString = asUtf8Ptr.toDartString();
  calloc.free(charPtr);
  return asDartString;
}

/// Returns a Dart String from a `char**`
///
/// n.b., THIS CONSUMES AND FREES THE POINTER
/// Do not use `charPtr` after this
///
/// For single pointers `char*` use `_getDartStringFromPtr`
String _getDartStringFromDoublePtr(
    ffi.Pointer<ffi.Pointer<ffi.Char>> doublePtr) {
  final asCharPtr = doublePtr.value.cast<ffi.Char>();
  final dstr = _getDartStringFromPtr(asCharPtr);
  calloc.free(doublePtr);
  return dstr;
}

ffi.Pointer<ffi.Char> _stringToFFIPointer(String s) {
  return s.toNativeUtf8().cast<ffi.Char>();
}

ffi.Pointer<ffi.Char> _hashMapToFFIPointer(Map<String, String> dict) {
  String val = dict.entries.map((e) => "${e.key}:${e.value}").join(';');
  return _stringToFFIPointer(val);
}

/// Returns the Dart equivalent of an empty `char**`
ffi.Pointer<ffi.Pointer<ffi.Char>> _getEmptyStringPointer() {
  return calloc<ffi.Pointer<ffi.Char>>();
}

/* Region: Utility Functions  */

/// Loads the dynamic library using an appropriate extension for the given platform
String getLibraryPath() {
  var libraryPath =
      path.join(Directory.current.path, 'libs', 'liboauthtool.so');
  if (Platform.isMacOS || Platform.isIOS) {
    libraryPath =
        path.join(Directory.current.path, 'libs', 'liboauthtool.dylib');
  } else if (Platform.isWindows) {
    libraryPath = path.join(Directory.current.path, 'libs', 'liboauthtool.dll');
  }
  return libraryPath;
}

/// Loads the dynamic library from the given library path
ffi.DynamicLibrary loadLibrary(String libraryPath) {
  final dylib = ffi.DynamicLibrary.open(libraryPath);
  return dylib;
}
Formatted 1 file (1 changed) in 0.52 seconds.


// DO NOT MODIFY THIS FILE
// This file contains automatically generated Dart Bindings.
// It was generated via the clang2src project, and ultimately comes from a set of annotated Rust source files
// Any modifications you make to this file will be reverted whenever this file is regenerated



 import 'package:ffi/ffi.dart';

 import 'dart:ffi' as ffi;

 import 'dart:io' show Platform, Directory;

 import 'package:path/path.dart' as path;



class liboauthtoolException implements Exception {
    final String msg;
    final int code;
    
    const liboauthtoolException(this.msg, this.code);
}

/* Region: C Constants */
const int C_NULL = 0;
const int C_TRUE = 1;
const int C_FALSE = 0;


/* Region: Dart Constants */


const int OAUTHTOOL_PASS = 0;


const int OAUTHTOOL_FAIL = 1;


const int OAUTHTOOL_FAIL_NULL_POINTER = 2;





/* Region: FFI Enums */

/// Code Challenge for PKCE-enabled Authorization Code Flow
/// https://www.rfc-editor.org/rfc/rfc7636#section-4.2
/// If PKCE is not enabled, use [CodeChallengeMethod.None]

enum CodeChallengeMethod {
    
    /// None, for when the code flow isnt PKCE enabled

    None(0),
    
    /// Plain: code_challenge=code_verifier

    Plain(1),
    
    /// Sha256: code_challenge = BASE64URL-ENCODE(SHA256(ASCII(code_verifier)))

    S256(2),
    
    ;

    final int value;

    const CodeChallengeMethod(this.value);
}

/// Specifies the different types of OAuth Flows
/// Implicit,
/// Client Credentials,
/// Authorization,
/// Authorization + PKCE,
/// Device

enum FlowType {
    
    /// Implicit Grant

    Implicit(0),
    
    /// Client Credentials Grant
/// Gives keys to an application without accessing user-specific resources

    ClientCredentials(1),
    
    /// Authorization Grant
/// Allows access to user-specific resources. Usually comes with an infinite-lifespan Refresh Token

    Authorization(2),
    
    /// Secure Authorization grant using the PKCE extension
/// Refresh Tokens are one-time-use

    AuthorizationPKCE(3),
    
    /// Flow for using a second device, which has a screen, to give access to a primary device, which does not have a screen

    Device(4),
    
    ;

    final int value;

    const FlowType(this.value);
}




/* Region: FFI Structs */


class C_Engine  extends  ffi.Opaque    {
    
}


class C_OAuth2Authorization  extends  ffi.Opaque    {
    
}


class C_OAuth2ClientCredentials  extends  ffi.Opaque    {
    
}


class C_OAuth2Implicit  extends  ffi.Opaque    {
    
}


class C_OAuth2PKCE  extends  ffi.Opaque    {
    
}


class C_FFIArray  extends  ffi.Struct    {
    
    /// Number of elements in the returned array

    @ffi.UintPtr()
    
     external int len;
    
    /// Max size of the array

    @ffi.UintPtr()
    
     external int cap;
    
    /// pointer to the first item in the array

    
    
     external ffi.Pointer<ffi.Pointer<ffi.Char>> arr;
    
}


class C_TokenResponse  extends  ffi.Struct    {
    
    /// If not null, contains a token that can be used to access the service

    
    
     external ffi.Pointer<ffi.Char> accessToken;
    
    /// If not null, contains a token that can be used to get a new access token

    
    
     external ffi.Pointer<ffi.Char> refreshToken;
    
    /// Seconds from received time that the token expires at

    @ffi.Int64()
    
     external int expiresAt;
    
    /// If not null, denotes what kind of token this is.  Usually Bearer

    
    
     external ffi.Pointer<ffi.Char> tokenType;
    
    
    
    
     external ffi.Pointer<C_FFIArray> scopes;
    
}


class C_AuthUrlOutput  extends  ffi.Struct    {
    
    
    
    
     external ffi.Pointer<ffi.Char> url;
    
    
    
    
     external ffi.Pointer<ffi.Char> localState;
    
    
    
    
     external ffi.Pointer<ffi.Char> pkceVerifierState;
    
}


class C_ParsedAuthorizationCode  extends  ffi.Struct    {
    
    /// Authorization Code. Always present.

    
    
     external ffi.Pointer<ffi.Char> code;
    
    /// State returned from server. Should match state given to server. Not always present

    
    
     external ffi.Pointer<ffi.Char> state;
    
}





/* Region: FFI Free Functions */

int ffi_encrypt(
    
    ffi.Pointer<ffi.Char> plain_text,
    
    ffi.Pointer<ffi.Pointer<ffi.Char>> encrypted_output,
    
    ffi.Pointer<C_Engine> engine,
    
    ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
    
) {
    
    return _ffi_encrypt(
        
        plain_text,
        
        encrypted_output,
        
        engine,
        
        err_ptr,
        
    );
    
}

final ffi_encryptPtr = _lookup<ffi.NativeFunction<ffi.Uint32 Function( ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Pointer<ffi.Char>>,  ffi.Pointer<C_Engine>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >>('encrypt');
final _ffi_encrypt = ffi_encryptPtr.asFunction<int Function( ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Pointer<ffi.Char>>,  ffi.Pointer<C_Engine>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >();

int ffi_decrypt(
    
    ffi.Pointer<ffi.Char> encrypted_text,
    
    ffi.Pointer<ffi.Pointer<ffi.Char>> decrypted_output,
    
    ffi.Pointer<C_Engine> engine,
    
    ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
    
) {
    
    return _ffi_decrypt(
        
        encrypted_text,
        
        decrypted_output,
        
        engine,
        
        err_ptr,
        
    );
    
}

final ffi_decryptPtr = _lookup<ffi.NativeFunction<ffi.Uint32 Function( ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Pointer<ffi.Char>>,  ffi.Pointer<C_Engine>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >>('decrypt');
final _ffi_decrypt = ffi_decryptPtr.asFunction<int Function( ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Pointer<ffi.Char>>,  ffi.Pointer<C_Engine>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >();

int ffi_OAuth2PKCE_new(
    
    ffi.Pointer<ffi.Pointer<C_OAuth2PKCE>> this_,
    
    ffi.Pointer<ffi.Char> client_id,
    
    ffi.Pointer<ffi.Char> client_secret,
    
    ffi.Pointer<ffi.Char> authorization_url,
    
    ffi.Pointer<ffi.Char> token_url,
    
    ffi.Pointer<ffi.Char> redirect_url,
    
    CodeChallengeMethod challenge_method,
    
    ffi.Pointer<ffi.Char> scopes,
    
    ffi.Pointer<ffi.Char> extra_parameters,
    
    int timeout_in_milliseconds,
    
    ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
    
) {
    
    return _ffi_OAuth2PKCE_new(
        
        this_,
        
        client_id,
        
        client_secret,
        
        authorization_url,
        
        token_url,
        
        redirect_url,
        
        challenge_method.value,
        
        scopes,
        
        extra_parameters,
        
        timeout_in_milliseconds,
        
        err_ptr,
        
    );
    
}

final ffi_OAuth2PKCE_newPtr = _lookup<ffi.NativeFunction<ffi.Uint32 Function( ffi.Pointer<ffi.Pointer<C_OAuth2PKCE>>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Int32,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Int64,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >>('OAuth2PKCE_new');
final _ffi_OAuth2PKCE_new = ffi_OAuth2PKCE_newPtr.asFunction<int Function( ffi.Pointer<ffi.Pointer<C_OAuth2PKCE>>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  int,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  int,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >();

int ffi_OAuth2PKCE_get_token_automatic(
    
    ffi.Pointer<C_OAuth2PKCE> this_,
    
    ffi.Pointer<ffi.Pointer<C_TokenResponse>> token_output,
    
    ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
    
) {
    
    return _ffi_OAuth2PKCE_get_token_automatic(
        
        this_,
        
        token_output,
        
        err_ptr,
        
    );
    
}

final ffi_OAuth2PKCE_get_token_automaticPtr = _lookup<ffi.NativeFunction<ffi.Uint32 Function( ffi.Pointer<C_OAuth2PKCE>,  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >>('OAuth2PKCE_get_token_automatic');
final _ffi_OAuth2PKCE_get_token_automatic = ffi_OAuth2PKCE_get_token_automaticPtr.asFunction<int Function( ffi.Pointer<C_OAuth2PKCE>,  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >();

int ffi_OAuth2PKCE_start_web_server_for_callback(
    
    ffi.Pointer<C_OAuth2PKCE> this_,
    
    ffi.Pointer<ffi.Char> redirect_url,
    
    ffi.Pointer<ffi.Char> verifier_state,
    
    ffi.Pointer<ffi.Pointer<C_TokenResponse>> token_output,
    
    int timeout_in_milliseconds,
    
    ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
    
) {
    
    return _ffi_OAuth2PKCE_start_web_server_for_callback(
        
        this_,
        
        redirect_url,
        
        verifier_state,
        
        token_output,
        
        timeout_in_milliseconds,
        
        err_ptr,
        
    );
    
}

final ffi_OAuth2PKCE_start_web_server_for_callbackPtr = _lookup<ffi.NativeFunction<ffi.Uint32 Function( ffi.Pointer<C_OAuth2PKCE>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,  ffi.Int64,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >>('OAuth2PKCE_start_web_server_for_callback');
final _ffi_OAuth2PKCE_start_web_server_for_callback = ffi_OAuth2PKCE_start_web_server_for_callbackPtr.asFunction<int Function( ffi.Pointer<C_OAuth2PKCE>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,  int,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >();

int ffi_OAuth2PKCE_get_authorization_url(
    
    ffi.Pointer<C_OAuth2PKCE> this_,
    
    ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>> authorization_output,
    
    ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
    
) {
    
    return _ffi_OAuth2PKCE_get_authorization_url(
        
        this_,
        
        authorization_output,
        
        err_ptr,
        
    );
    
}

final ffi_OAuth2PKCE_get_authorization_urlPtr = _lookup<ffi.NativeFunction<ffi.Uint32 Function( ffi.Pointer<C_OAuth2PKCE>,  ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >>('OAuth2PKCE_get_authorization_url');
final _ffi_OAuth2PKCE_get_authorization_url = ffi_OAuth2PKCE_get_authorization_urlPtr.asFunction<int Function( ffi.Pointer<C_OAuth2PKCE>,  ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >();

int ffi_OAuth2PKCE_exchange_authorization_code_for_token(
    
    ffi.Pointer<C_OAuth2PKCE> this_,
    
    ffi.Pointer<ffi.Char> authorization_code,
    
    ffi.Pointer<ffi.Char> verifier_state,
    
    ffi.Pointer<ffi.Pointer<C_TokenResponse>> token_output,
    
    ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
    
) {
    
    return _ffi_OAuth2PKCE_exchange_authorization_code_for_token(
        
        this_,
        
        authorization_code,
        
        verifier_state,
        
        token_output,
        
        err_ptr,
        
    );
    
}

final ffi_OAuth2PKCE_exchange_authorization_code_for_tokenPtr = _lookup<ffi.NativeFunction<ffi.Uint32 Function( ffi.Pointer<C_OAuth2PKCE>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >>('OAuth2PKCE_exchange_authorization_code_for_token');
final _ffi_OAuth2PKCE_exchange_authorization_code_for_token = ffi_OAuth2PKCE_exchange_authorization_code_for_tokenPtr.asFunction<int Function( ffi.Pointer<C_OAuth2PKCE>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >();

int ffi_OAuth2PKCE_refresh_access_token(
    
    ffi.Pointer<C_OAuth2PKCE> this_,
    
    ffi.Pointer<ffi.Char> refresh_token,
    
    ffi.Pointer<ffi.Pointer<C_TokenResponse>> token_output,
    
    ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
    
) {
    
    return _ffi_OAuth2PKCE_refresh_access_token(
        
        this_,
        
        refresh_token,
        
        token_output,
        
        err_ptr,
        
    );
    
}

final ffi_OAuth2PKCE_refresh_access_tokenPtr = _lookup<ffi.NativeFunction<ffi.Uint32 Function( ffi.Pointer<C_OAuth2PKCE>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >>('OAuth2PKCE_refresh_access_token');
final _ffi_OAuth2PKCE_refresh_access_token = ffi_OAuth2PKCE_refresh_access_tokenPtr.asFunction<int Function( ffi.Pointer<C_OAuth2PKCE>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >();

int ffi_OAuth2Authorization_new(
    
    ffi.Pointer<ffi.Pointer<C_OAuth2Authorization>> this_,
    
    ffi.Pointer<ffi.Char> client_id,
    
    ffi.Pointer<ffi.Char> client_secret,
    
    ffi.Pointer<ffi.Char> authorization_url,
    
    ffi.Pointer<ffi.Char> token_url,
    
    ffi.Pointer<ffi.Char> redirect_url,
    
    ffi.Pointer<ffi.Char> scopes,
    
    ffi.Pointer<ffi.Char> extra_parameters,
    
    int timeout_in_milliseconds,
    
    ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
    
) {
    
    return _ffi_OAuth2Authorization_new(
        
        this_,
        
        client_id,
        
        client_secret,
        
        authorization_url,
        
        token_url,
        
        redirect_url,
        
        scopes,
        
        extra_parameters,
        
        timeout_in_milliseconds,
        
        err_ptr,
        
    );
    
}

final ffi_OAuth2Authorization_newPtr = _lookup<ffi.NativeFunction<ffi.Uint32 Function( ffi.Pointer<ffi.Pointer<C_OAuth2Authorization>>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Int64,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >>('OAuth2Authorization_new');
final _ffi_OAuth2Authorization_new = ffi_OAuth2Authorization_newPtr.asFunction<int Function( ffi.Pointer<ffi.Pointer<C_OAuth2Authorization>>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Char>,  int,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >();

int ffi_OAuth2Authorization_get_authorization_url(
    
    ffi.Pointer<C_OAuth2Authorization> this_,
    
    ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>> authorization_output,
    
    ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
    
) {
    
    return _ffi_OAuth2Authorization_get_authorization_url(
        
        this_,
        
        authorization_output,
        
        err_ptr,
        
    );
    
}

final ffi_OAuth2Authorization_get_authorization_urlPtr = _lookup<ffi.NativeFunction<ffi.Uint32 Function( ffi.Pointer<C_OAuth2Authorization>,  ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >>('OAuth2Authorization_get_authorization_url');
final _ffi_OAuth2Authorization_get_authorization_url = ffi_OAuth2Authorization_get_authorization_urlPtr.asFunction<int Function( ffi.Pointer<C_OAuth2Authorization>,  ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >();

int ffi_OAuth2Authorization_exchange_authorization_code_for_token(
    
    ffi.Pointer<C_OAuth2Authorization> this_,
    
    ffi.Pointer<ffi.Char> authorization_code,
    
    ffi.Pointer<ffi.Pointer<C_TokenResponse>> token_output,
    
    ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
    
) {
    
    return _ffi_OAuth2Authorization_exchange_authorization_code_for_token(
        
        this_,
        
        authorization_code,
        
        token_output,
        
        err_ptr,
        
    );
    
}

final ffi_OAuth2Authorization_exchange_authorization_code_for_tokenPtr = _lookup<ffi.NativeFunction<ffi.Uint32 Function( ffi.Pointer<C_OAuth2Authorization>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >>('OAuth2Authorization_exchange_authorization_code_for_token');
final _ffi_OAuth2Authorization_exchange_authorization_code_for_token = ffi_OAuth2Authorization_exchange_authorization_code_for_tokenPtr.asFunction<int Function( ffi.Pointer<C_OAuth2Authorization>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >();

int ffi_OAuth2Authorization_refresh_access_token(
    
    ffi.Pointer<C_OAuth2Authorization> this_,
    
    ffi.Pointer<ffi.Char> refresh_token,
    
    ffi.Pointer<ffi.Pointer<C_TokenResponse>> token_output,
    
    ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
    
) {
    
    return _ffi_OAuth2Authorization_refresh_access_token(
        
        this_,
        
        refresh_token,
        
        token_output,
        
        err_ptr,
        
    );
    
}

final ffi_OAuth2Authorization_refresh_access_tokenPtr = _lookup<ffi.NativeFunction<ffi.Uint32 Function( ffi.Pointer<C_OAuth2Authorization>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >>('OAuth2Authorization_refresh_access_token');
final _ffi_OAuth2Authorization_refresh_access_token = ffi_OAuth2Authorization_refresh_access_tokenPtr.asFunction<int Function( ffi.Pointer<C_OAuth2Authorization>,  ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Pointer<C_TokenResponse>>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >();

int ffi_parse_authorization_callback_url(
    
    ffi.Pointer<ffi.Char> filled_callback_url,
    
    ffi.Pointer<ffi.Pointer<C_ParsedAuthorizationCode>> parsed_authorization_code_output,
    
    ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
    
) {
    
    return _ffi_parse_authorization_callback_url(
        
        filled_callback_url,
        
        parsed_authorization_code_output,
        
        err_ptr,
        
    );
    
}

final ffi_parse_authorization_callback_urlPtr = _lookup<ffi.NativeFunction<ffi.Uint32 Function( ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Pointer<C_ParsedAuthorizationCode>>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >>('parse_authorization_callback_url');
final _ffi_parse_authorization_callback_url = ffi_parse_authorization_callback_urlPtr.asFunction<int Function( ffi.Pointer<ffi.Char>,  ffi.Pointer<ffi.Pointer<C_ParsedAuthorizationCode>>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >();

int ffi_Engine_new(
    
    ffi.Pointer<ffi.Pointer<C_Engine>> engine,
    
    ffi.Pointer<ffi.Pointer<ffi.Char>> err_ptr,
    
) {
    
    return _ffi_Engine_new(
        
        engine,
        
        err_ptr,
        
    );
    
}

final ffi_Engine_newPtr = _lookup<ffi.NativeFunction<ffi.Uint32 Function( ffi.Pointer<ffi.Pointer<C_Engine>>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >>('Engine_new');
final _ffi_Engine_new = ffi_Engine_newPtr.asFunction<int Function( ffi.Pointer<ffi.Pointer<C_Engine>>,  ffi.Pointer<ffi.Pointer<ffi.Char>>, ) >();

void ffi_Engine_free(
    
    ffi.Pointer<C_Engine> engine,
    
) {
    
    return _ffi_Engine_free(
        
        engine,
        
    );
    
}

final ffi_Engine_freePtr = _lookup<ffi.NativeFunction<ffi.Void Function( ffi.Pointer<C_Engine>, ) >>('Engine_free');
final _ffi_Engine_free = ffi_Engine_freePtr.asFunction<void Function( ffi.Pointer<C_Engine>, ) >();

void ffi_OAuth2PKCE_free(
    
    ffi.Pointer<C_OAuth2PKCE> mgr,
    
) {
    
    return _ffi_OAuth2PKCE_free(
        
        mgr,
        
    );
    
}

final ffi_OAuth2PKCE_freePtr = _lookup<ffi.NativeFunction<ffi.Void Function( ffi.Pointer<C_OAuth2PKCE>, ) >>('OAuth2PKCE_free');
final _ffi_OAuth2PKCE_free = ffi_OAuth2PKCE_freePtr.asFunction<void Function( ffi.Pointer<C_OAuth2PKCE>, ) >();

void ffi_OAuth2Authorization_free(
    
    ffi.Pointer<C_OAuth2Authorization> mgr,
    
) {
    
    return _ffi_OAuth2Authorization_free(
        
        mgr,
        
    );
    
}

final ffi_OAuth2Authorization_freePtr = _lookup<ffi.NativeFunction<ffi.Void Function( ffi.Pointer<C_OAuth2Authorization>, ) >>('OAuth2Authorization_free');
final _ffi_OAuth2Authorization_free = ffi_OAuth2Authorization_freePtr.asFunction<void Function( ffi.Pointer<C_OAuth2Authorization>, ) >();

void ffi_OAuth2ClientCredentials_free(
    
    ffi.Pointer<C_OAuth2ClientCredentials> mgr,
    
) {
    
    return _ffi_OAuth2ClientCredentials_free(
        
        mgr,
        
    );
    
}

final ffi_OAuth2ClientCredentials_freePtr = _lookup<ffi.NativeFunction<ffi.Void Function( ffi.Pointer<C_OAuth2ClientCredentials>, ) >>('OAuth2ClientCredentials_free');
final _ffi_OAuth2ClientCredentials_free = ffi_OAuth2ClientCredentials_freePtr.asFunction<void Function( ffi.Pointer<C_OAuth2ClientCredentials>, ) >();

void ffi_OAuth2Implicit_free(
    
    ffi.Pointer<C_OAuth2Implicit> mgr,
    
) {
    
    return _ffi_OAuth2Implicit_free(
        
        mgr,
        
    );
    
}

final ffi_OAuth2Implicit_freePtr = _lookup<ffi.NativeFunction<ffi.Void Function( ffi.Pointer<C_OAuth2Implicit>, ) >>('OAuth2Implicit_free');
final _ffi_OAuth2Implicit_free = ffi_OAuth2Implicit_freePtr.asFunction<void Function( ffi.Pointer<C_OAuth2Implicit>, ) >();

ffi.Pointer<C_FFIArray> ffi_TestGetRustStringList(
    
) {
    
    return _ffi_TestGetRustStringList(
        
    );
    
}

final ffi_TestGetRustStringListPtr = _lookup<ffi.NativeFunction<ffi.Pointer<C_FFIArray> Function() >>('TestGetRustStringList');
final _ffi_TestGetRustStringList = ffi_TestGetRustStringListPtr.asFunction<ffi.Pointer<C_FFIArray> Function() >();





/* Region: Dart Classes for use by the end-user */
    
class Engine  implements  IWithPtr,   ffi.Finalizable    {
    
    /* Fields */
    
    
    
    ffi.Pointer<C_Engine> _selfPtr;
    
    /// Pointer to the backing `free` function which disposes the backing pointer when this dart object is GC'd
    
     static  final ffi.NativeFinalizer _finalizer = ffi.NativeFinalizer(ffi_Engine_freePtr.cast());
    
    

    
    
    /* Constructors */
    Engine._(this._selfPtr) {
        _finalizer.attach(this, _selfPtr.cast(), detach: this);
    }
    

    
    /* Functions */
    
        
        factory    Engine._fromCPointerPointer( ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr, ) {
            
        
                ffi.Pointer<ffi.Pointer<C_Engine>> ptr = voidPtr.cast();
                final _innerPtr = ptr.value;
                return Engine._(_innerPtr);
                    
        
    }
    
        @override
          ffi.Pointer<ffi.Void>  getPointer() {
            
        return _selfPtr.cast();
        
    }
    
    
}
    
class OAuth2Authorization  implements  IWithPtr,   ffi.Finalizable    {
    
    /* Fields */
    
    
    
    ffi.Pointer<C_OAuth2Authorization> _selfPtr;
    
    /// Pointer to the backing `free` function which disposes the backing pointer when this dart object is GC'd
    
     static  final ffi.NativeFinalizer _finalizer = ffi.NativeFinalizer(ffi_OAuth2Authorization_freePtr.cast());
    
    

    
    
    /* Constructors */
    OAuth2Authorization._(this._selfPtr) {
        _finalizer.attach(this, _selfPtr.cast(), detach: this);
    }
    

    
    /* Functions */
    
        
        factory    OAuth2Authorization._fromCPointerPointer( ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr, ) {
            
        
                ffi.Pointer<ffi.Pointer<C_OAuth2Authorization>> ptr = voidPtr.cast();
                final _innerPtr = ptr.value;
                return OAuth2Authorization._(_innerPtr);
                    
        
    }
    
        @override
          ffi.Pointer<ffi.Void>  getPointer() {
            
        return _selfPtr.cast();
        
    }
    
    
}
    
class OAuth2ClientCredentials  implements  IWithPtr,   ffi.Finalizable    {
    
    /* Fields */
    
    
    
    ffi.Pointer<C_OAuth2ClientCredentials> _selfPtr;
    
    /// Pointer to the backing `free` function which disposes the backing pointer when this dart object is GC'd
    
     static  final ffi.NativeFinalizer _finalizer = ffi.NativeFinalizer(ffi_OAuth2ClientCredentials_freePtr.cast());
    
    

    
    
    /* Constructors */
    OAuth2ClientCredentials._(this._selfPtr) {
        _finalizer.attach(this, _selfPtr.cast(), detach: this);
    }
    

    
    /* Functions */
    
        
        factory    OAuth2ClientCredentials._fromCPointerPointer( ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr, ) {
            
        
                ffi.Pointer<ffi.Pointer<C_OAuth2ClientCredentials>> ptr = voidPtr.cast();
                final _innerPtr = ptr.value;
                return OAuth2ClientCredentials._(_innerPtr);
                    
        
    }
    
        @override
          ffi.Pointer<ffi.Void>  getPointer() {
            
        return _selfPtr.cast();
        
    }
    
    
}
    
class OAuth2Implicit  implements  IWithPtr,   ffi.Finalizable    {
    
    /* Fields */
    
    
    
    ffi.Pointer<C_OAuth2Implicit> _selfPtr;
    
    /// Pointer to the backing `free` function which disposes the backing pointer when this dart object is GC'd
    
     static  final ffi.NativeFinalizer _finalizer = ffi.NativeFinalizer(ffi_OAuth2Implicit_freePtr.cast());
    
    

    
    
    /* Constructors */
    OAuth2Implicit._(this._selfPtr) {
        _finalizer.attach(this, _selfPtr.cast(), detach: this);
    }
    

    
    /* Functions */
    
        
        factory    OAuth2Implicit._fromCPointerPointer( ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr, ) {
            
        
                ffi.Pointer<ffi.Pointer<C_OAuth2Implicit>> ptr = voidPtr.cast();
                final _innerPtr = ptr.value;
                return OAuth2Implicit._(_innerPtr);
                    
        
    }
    
        @override
          ffi.Pointer<ffi.Void>  getPointer() {
            
        return _selfPtr.cast();
        
    }
    
    
}
    
class OAuth2PKCE  implements  IWithPtr,   ffi.Finalizable    {
    
    /* Fields */
    
    
    
    ffi.Pointer<C_OAuth2PKCE> _selfPtr;
    
    /// Pointer to the backing `free` function which disposes the backing pointer when this dart object is GC'd
    
     static  final ffi.NativeFinalizer _finalizer = ffi.NativeFinalizer(ffi_OAuth2PKCE_freePtr.cast());
    
    

    
    
    /* Constructors */
    OAuth2PKCE._(this._selfPtr) {
        _finalizer.attach(this, _selfPtr.cast(), detach: this);
    }
    

    
    /* Functions */
    
        
        factory    OAuth2PKCE._fromCPointerPointer( ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr, ) {
            
        
                ffi.Pointer<ffi.Pointer<C_OAuth2PKCE>> ptr = voidPtr.cast();
                final _innerPtr = ptr.value;
                return OAuth2PKCE._(_innerPtr);
                    
        
    }
    
        @override
          ffi.Pointer<ffi.Void>  getPointer() {
            
        return _selfPtr.cast();
        
    }
    
    
}
    
class FFIArray   {
    
    /* Fields */
    
    /// Number of elements in the returned array

    
     final int len;
    
    /// Max size of the array

    
     final int cap;
    
    /// pointer to the first item in the array

    
     final ffi.Pointer<ffi.Pointer<ffi.Char>> arr;
    
    

    
    

    
    /* Functions */
    
        
        factory    FFIArray._fromCPointerPointer( ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr, ) {
            
        
                ffi.Pointer<ffi.Pointer<C_FFIArray>> ptr = voidPtr.cast();
                final st = ptr.value.ref;
                return FFIArray._fromCStruct(st);
        
    }
    
        
        factory    FFIArray._fromCStruct( C_FFIArray c, ) {
            
        'TODO: ayy lmao is struct and has fields';
        
    }
    
    
}
    
class TokenResponse   {
    
    /* Fields */
    
    /// If not null, contains a token that can be used to access the service

    
     final ffi.Pointer<ffi.Char> accessToken;
    
    /// If not null, contains a token that can be used to get a new access token

    
     final ffi.Pointer<ffi.Char>? refreshToken;
    
    /// Seconds from received time that the token expires at

    
     final int expiresAt;
    
    /// If not null, denotes what kind of token this is.  Usually Bearer

    
     final ffi.Pointer<ffi.Char>? tokenType;
    
    
    
     final ffi.Pointer<C_FFIArray> scopes;
    
    

    
    

    
    /* Functions */
    
        
        factory    TokenResponse._fromCPointerPointer( ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr, ) {
            
        
                ffi.Pointer<ffi.Pointer<C_TokenResponse>> ptr = voidPtr.cast();
                final st = ptr.value.ref;
                return TokenResponse._fromCStruct(st);
        
    }
    
        
        factory    TokenResponse._fromCStruct( C_TokenResponse c, ) {
            
        'TODO: ayy lmao is struct and has fields';
        
    }
    
    
}
    
class AuthUrlOutput   {
    
    /* Fields */
    
    
    
     final ffi.Pointer<ffi.Char> url;
    
    
    
     final ffi.Pointer<ffi.Char>? localState;
    
    
    
     final ffi.Pointer<ffi.Char>? pkceVerifierState;
    
    

    
    

    
    /* Functions */
    
        
        factory    AuthUrlOutput._fromCPointerPointer( ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr, ) {
            
        
                ffi.Pointer<ffi.Pointer<C_AuthUrlOutput>> ptr = voidPtr.cast();
                final st = ptr.value.ref;
                return AuthUrlOutput._fromCStruct(st);
        
    }
    
        
        factory    AuthUrlOutput._fromCStruct( C_AuthUrlOutput c, ) {
            
        'TODO: ayy lmao is struct and has fields';
        
    }
    
    
}
    
class ParsedAuthorizationCode   {
    
    /* Fields */
    
    /// Authorization Code. Always present.

    
     final ffi.Pointer<ffi.Char> code;
    
    /// State returned from server. Should match state given to server. Not always present

    
     final ffi.Pointer<ffi.Char>? state;
    
    

    
    

    
    /* Functions */
    
        
        factory    ParsedAuthorizationCode._fromCPointerPointer( ffi.Pointer<ffi.Pointer<ffi.Void>> voidPtr, ) {
            
        
                ffi.Pointer<ffi.Pointer<C_ParsedAuthorizationCode>> ptr = voidPtr.cast();
                final st = ptr.value.ref;
                return ParsedAuthorizationCode._fromCStruct(st);
        
    }
    
        
        factory    ParsedAuthorizationCode._fromCStruct( C_ParsedAuthorizationCode c, ) {
            
        'TODO: ayy lmao is struct and has fields';
        
    }
    
    
}
    




/* Region: Dart Free Functions */


String encrypt( String plain_text,  Engine engine, ) {
    
        
    /* Get error pointer in case function returns failure */
    final ffi.Pointer<ffi.Pointer<ffi.Char>> cErrPtr = _getPointerForType<String>().cast();

            
    /* get pointer types for items that require it*/
                
                    
    final cplain_textPtr = _getPointerForData(plain_text);
                    
                
                    
    final cenginePtr = _getPointerForData(engine);
                    
                
            

            
    /* get Output Pointer type */
    final cencryptOutputPtr = _getPointerForType<String>();
            

    /* call native function */
    int errCode = ffi_encrypt(
            
        cencryptOutputPtr.cast(),
            
            
                 cplain_textPtr.cast(),
            
                 cenginePtr.cast(),
            
        cErrPtr,
    );

    /* Check error code */
    if (errCode != C_FALSE) {
        /* free pointers if required  */
        
            
        calloc.free(cplain_textPtr);
            
        
            
        calloc.free(cenginePtr);
            
        
        
        calloc.free(cencryptOutputPtr);
        

        /* throw final Exception */
        throw liboauthtoolException(_getDartStringFromDoublePtr(cErrPtr), errCode);
    }

    /* Free allocated pointers */
        
            
    calloc.free(cplain_textPtr);
            
        
            
    calloc.free(cenginePtr);
            
        
        
    calloc.free(cErrPtr);
        

    /* return final value */
    
    
}


String decrypt( String encrypted_text,  Engine engine, ) {
    
        
    /* Get error pointer in case function returns failure */
    final ffi.Pointer<ffi.Pointer<ffi.Char>> cErrPtr = _getPointerForType<String>().cast();

            
    /* get pointer types for items that require it*/
                
                    
    final cencrypted_textPtr = _getPointerForData(encrypted_text);
                    
                
                    
    final cenginePtr = _getPointerForData(engine);
                    
                
            

            
    /* get Output Pointer type */
    final cdecryptOutputPtr = _getPointerForType<String>();
            

    /* call native function */
    int errCode = ffi_decrypt(
            
        cdecryptOutputPtr.cast(),
            
            
                 cencrypted_textPtr.cast(),
            
                 cenginePtr.cast(),
            
        cErrPtr,
    );

    /* Check error code */
    if (errCode != C_FALSE) {
        /* free pointers if required  */
        
            
        calloc.free(cencrypted_textPtr);
            
        
            
        calloc.free(cenginePtr);
            
        
        
        calloc.free(cdecryptOutputPtr);
        

        /* throw final Exception */
        throw liboauthtoolException(_getDartStringFromDoublePtr(cErrPtr), errCode);
    }

    /* Free allocated pointers */
        
            
    calloc.free(cencrypted_textPtr);
            
        
            
    calloc.free(cenginePtr);
            
        
        
    calloc.free(cErrPtr);
        

    /* return final value */
    
    
}


ParsedAuthorizationCode parse_authorization_callback_url( String filled_callback_url, ) {
    
        
    /* Get error pointer in case function returns failure */
    final ffi.Pointer<ffi.Pointer<ffi.Char>> cErrPtr = _getPointerForType<String>().cast();

            
    /* get pointer types for items that require it*/
                
                    
    final cfilled_callback_urlPtr = _getPointerForData(filled_callback_url);
                    
                
            

            
    /* get Output Pointer type */
    final cparse_authorization_callback_urlOutputPtr = _getPointerForType<ParsedAuthorizationCode>();
            

    /* call native function */
    int errCode = ffi_parse_authorization_callback_url(
            
        cparse_authorization_callback_urlOutputPtr.cast(),
            
            
                 cfilled_callback_urlPtr.cast(),
            
        cErrPtr,
    );

    /* Check error code */
    if (errCode != C_FALSE) {
        /* free pointers if required  */
        
            
        calloc.free(cfilled_callback_urlPtr);
            
        
        
        calloc.free(cparse_authorization_callback_urlOutputPtr);
        

        /* throw final Exception */
        throw liboauthtoolException(_getDartStringFromDoublePtr(cErrPtr), errCode);
    }

    /* Free allocated pointers */
        
            
    calloc.free(cfilled_callback_urlPtr);
            
        
        
    calloc.free(cErrPtr);
        

    /* return final value */
    
    
}


ffi.Pointer<C_FFIArray> TestGetRustStringList() {
    
        
    'lol?';
    
    
}



/* Region: Dart Pointer Utility Functions  */

/// Interface to get a Pointer to the backing data on classes that 
/// cross FFI boundaries
abstract class IWithPtr {
    ffi.Pointer<ffi.Void> getPointer();
}

/// Holds the symbol lookup function.
final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName) _lookup = loadLibrary(getLibraryPath()).lookup;


T transformFromPointer<T, E extends ffi.NativeType>(ffi.Pointer<ffi.Pointer<E>> data) {
    if (T == String) {
      return _getDartStringFromDoublePtr(data.cast()) as T;
    } 

    
    else if(T == C_Engine) {
        return Engine._fromCPointerPointer(data.cast()) as T;
    }
    
    else if(T == C_OAuth2Authorization) {
        return OAuth2Authorization._fromCPointerPointer(data.cast()) as T;
    }
    
    else if(T == C_OAuth2ClientCredentials) {
        return OAuth2ClientCredentials._fromCPointerPointer(data.cast()) as T;
    }
    
    else if(T == C_OAuth2Implicit) {
        return OAuth2Implicit._fromCPointerPointer(data.cast()) as T;
    }
    
    else if(T == C_OAuth2PKCE) {
        return OAuth2PKCE._fromCPointerPointer(data.cast()) as T;
    }
    
    else if(T == C_FFIArray) {
        return FFIArray._fromCPointerPointer(data.cast()) as T;
    }
    
    else if(T == C_TokenResponse) {
        return TokenResponse._fromCPointerPointer(data.cast()) as T;
    }
    
    else if(T == C_AuthUrlOutput) {
        return AuthUrlOutput._fromCPointerPointer(data.cast()) as T;
    }
    
    else if(T == C_ParsedAuthorizationCode) {
        return ParsedAuthorizationCode._fromCPointerPointer(data.cast()) as T;
    }
    
    throw liboauthtoolException('Invalid data in transformFromPointer: $T', -4);
  }

ffi.Pointer<ffi.Void> _getPointerForType<T>() {
    if (T == String) {
      return _getEmptyStringPointer().cast();
    } 
    
    else if(T == Engine) {
        return calloc<ffi.Pointer<C_Engine>>().cast();
    }
    
    else if(T == OAuth2Authorization) {
        return calloc<ffi.Pointer<C_OAuth2Authorization>>().cast();
    }
    
    else if(T == OAuth2ClientCredentials) {
        return calloc<ffi.Pointer<C_OAuth2ClientCredentials>>().cast();
    }
    
    else if(T == OAuth2Implicit) {
        return calloc<ffi.Pointer<C_OAuth2Implicit>>().cast();
    }
    
    else if(T == OAuth2PKCE) {
        return calloc<ffi.Pointer<C_OAuth2PKCE>>().cast();
    }
    
    else if(T == FFIArray) {
        return calloc<ffi.Pointer<C_FFIArray>>().cast();
    }
    
    else if(T == TokenResponse) {
        return calloc<ffi.Pointer<C_TokenResponse>>().cast();
    }
    
    else if(T == AuthUrlOutput) {
        return calloc<ffi.Pointer<C_AuthUrlOutput>>().cast();
    }
    
    else if(T == ParsedAuthorizationCode) {
        return calloc<ffi.Pointer<C_ParsedAuthorizationCode>>().cast();
    }
    
    else {
      throw liboauthtoolException('Invalid type: $T', -3);
    }
  }

  /// Returns a castable pointer based on the input data.
/// This function is only valid for Types [String, {custom C generated classes}]
/// Will throw an Exception if passed invalid types
ffi.Pointer<ffi.Void> _getPointerForData(dynamic data) {
    if (data is String) {
      return _stringToFFIPointer(data).cast();
    } else if (data is IWithPtr) {
      return data.getPointer().cast();
    } else {
      throw liboauthtoolException('Invalid data type for pointer: $data', -2);
    }
  }


/// Returns a Dart String from a `char*`
///
/// n.b., THIS CONSUMES AND FREES THE POINTER
/// Do not use `charPtr` after this
///
/// For double pointers `char**` use `_getDartStringFromDoublePtr`
String _getDartStringFromPtr(ffi.Pointer<ffi.Char> charPtr) {
  final asUtf8Ptr = charPtr.cast<Utf8>();
  final asDartString = asUtf8Ptr.toDartString();
  calloc.free(charPtr);
  return asDartString;
}

/// Returns a Dart String from a `char**`
///
/// n.b., THIS CONSUMES AND FREES THE POINTER
/// Do not use `charPtr` after this
///
/// For single pointers `char*` use `_getDartStringFromPtr`
String _getDartStringFromDoublePtr(ffi.Pointer<ffi.Pointer<ffi.Char>> doublePtr) {
  final asCharPtr = doublePtr.value.cast<ffi.Char>();
  final dstr = _getDartStringFromPtr(asCharPtr);
  calloc.free(doublePtr);
  return dstr;
}

ffi.Pointer<ffi.Char> _stringToFFIPointer(String s) {
  return s.toNativeUtf8().cast<ffi.Char>();
}

ffi.Pointer<ffi.Char> _hashMapToFFIPointer(Map<String, String> dict) {
  String val = dict.entries.map((e) => "${e.key}:${e.value}").join(';');
  return _stringToFFIPointer(val);
}

/// Returns the Dart equivalent of an empty `char**`
ffi.Pointer<ffi.Pointer<ffi.Char>> _getEmptyStringPointer() {
  return calloc<ffi.Pointer<ffi.Char>>();
}

/* Region: Utility Functions  */

/// Loads the dynamic library using an appropriate extension for the given platform
String getLibraryPath() {
  var libraryPath = path.join(Directory.current.path, 'libs', 'liboauthtool.so');
  if (Platform.isMacOS || Platform.isIOS) {
    libraryPath = path.join(Directory.current.path, 'libs', 'liboauthtool.dylib');
  } else if (Platform.isWindows) {
    libraryPath = path.join(Directory.current.path, 'libs', 'liboauthtool.dll');
  }
  return libraryPath;
}

/// Loads the dynamic library from the given library path
ffi.DynamicLibrary loadLibrary(String libraryPath) {
  final dylib = ffi.DynamicLibrary.open(libraryPath);
  return dylib;
}

