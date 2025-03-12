app = COSMIC 설정

dbus-connection-error = DBus 연결 실패
ok = OK
unknown = Unknown

number = { $number }

## Network & Wireless

connections-and-profiles = { $variant ->
    [wired] Wired
    [wifi] Wi-Fi
    [vpn] VPN
    *[other] Unknown
} 연결 및 연결 설정.

add-network = 네트워크 추가
    .profile = 프로파일 추카
add-vpn = VPN 추가
airplane-on = 에어플레인 모드 활성화.
cable-unplugged = 유선 연결 해제
connect = 연결
connected = 연결됨
connecting = 연결중…
disconnect = 연결해제
forget = 삭제
known-networks = 저장된 네트워크
network-and-wireless = 네트워크 & Network & Wireless
no-networks = 네트워크를 찾을 수 없음.
no-vpn = 연결 가능한 VPN 존재하지 않음.
password = 패스워드
remove = 제거
settings = 설정
username = 사용자명
visible-networks = Visible Networks

auth-dialog = 인증 요청
    .vpn-description = VPN 서비스를 위해 사용자명과 패스워드를 입력하세요.
    .wifi-description = 패스워드나 암호화 키를 입력하세요. 라우터에서 "WPS" 크를 눌러 연결할 수도 있습니다.

forget-dialog = 이 Wi-Fi 네트워크를 삭제할까요?
    .description = 다음에 이 Wi-Fi 네트워크를 이용하려면 패스워드를 다시 입력해야 합니다.

network-device-state =
    .activated = 연결됨
    .config = 연결중
    .deactivating = 연결해제중
    .disconnected = 연결해제됨
    .failed = 연결 실패함
    .ip-check = 연결 확인중
    .ip-config = IP와 라우팅 정보 요청중
    .need-auth = 인증 필요
    .prepare = 연결 준비중
    .secondaries = 재 연결 대기중
    .unavailable = 사용불가
    .unknown = 알 수 없는 상태
    .unmanaged = 관리되지 않음
    .unplugged = 케이블 연결끊임

remove-connection-dialog = 연결 프로파일 삭제
    .vpn-description = 다음에 이 네트워크를 사용하려면 패스워드를 다시 입력해야 합니다.
    .wired-description = 다음에 다시 사용하려면 프로파일을 다시 생성해야 합니다.
    
vpn = VPN
    .connections = VPN 연결
    .error = VPN 설정 추가 실패함
    .remove = 연결 프로파일 제거
    .select-file = VPN 설정 파일 선택

vpn-error = VPN 오류
    .config = VPN 설정 추가 실패함
    .connect = VPN 연결 실패
    .connection-editor = 연결 설정(editor) 실패
    .connection-settings = 활성화된 연결 설정 가져오기 실패
    .updating-state = 네트워크 관리 상태 갱신 실패
    .wireguard-config-path = WireGuard 설정을 위한 파일 경로가 옮바르지 않음
    .wireguard-config-path-desc = 선택한 파일은 로컬 파일 시스템이어야 합니다.
    .wireguard-device = WireGuard 장치 생성 실패
    .with-password = nmcli와 VPN { $field ->
        *[username] 사용자명
        [password] 패스워드
        [password-flags] password-flags
    } 설정 실패

wired = 유선
    .adapter = 유선 adapter { $id }
    .connections = 유선 연결
    .devices = 유선 장비
    .remove = 연결 프로파일 제거

wifi = Wi-Fi
    .adapter = Wi-Fi adapter { $id }
    .forget = 이 네트워크 제거

wireguard-dialog = WireGuard 장치 추가
    .description = WireGuard 설정을 위한 장치명 선택

## Networking: Online Accounts

online-accounts = 온라인 계정
    .desc = 계정, IMAP, SMTP, 기업 로그인 추가

# Bluetooth

confirm = 확정

bluetooth = 블루투스
    .desc = 블루투스 장치 관리
    .status = 이 장치는 블루투스 설정이 열려있는 동안 { $aliases } 로 보여집니다.
    .connected = 연결됨
    .connecting = 연결중
    .disconnecting = 연결해제중
    .connect = 연결
    .disconnect = 연결해제
    .forget = 삭제
    .dbus-error = DBus와 통신중 에러가 발생했습니다. :{ $why }

bluetooth-paired = 이전에 연결된 장치들
    .connect = 연결
    .battery = { $percentage }% 배터리

bluetooth-confirm-pin = 블루투스 PIN 확정
    .description = 다음 PIN이 { $device } 에 표시된 것과 일치하는지 확인하세요.

bluetooth-available = 사용 가능한 장치

bluetooth-adapters = 블루투스 어탭터

## Accessibility

accessibility = 접근성
    .vision = 비전
    .on = 커짐
    .off = 꺼짐
    .unavailable = 사용할수 없음
magnifier = 돋보기
    .controls =
        또는 키보드 단축기를 사용합니다:
        Super + = 확대합니다,
        Super + - 축소합니다,
        Super + 마우스로 스크롤합니다.
    .increment = 줌 확대
    .signin = 로그인 시 돋보기 시작
    .applet = 패널의 애플릿에서 돋보기 켜기/끄기 선택
    .movement = 줌 영역 이동
    .continuous = 포인터와 연속적으로 사용
    .onedge = 포인트가 모서리에 도달했을때
    .centered = 포인터를 중앙에 유지

## Desktop

desktop = 데스트톱

## Desktop: Wallpaper

wallpaper = 배경화면
    .change = 항상 이미지 변경`
    .desc = 배경화면 이미지, 색상, 슬라이드쇼 옵션
    .fit = 배경화면 맞춤
    .folder-dialog = 배경화면 폴더 선택
    .image-dialog = 배경화면 이미지 선택
    .plural = 배경화면
    .same = 모든 디스플레이에서 동일한 배경화면
    .slide = 슬라이드쇼

add-color = 색상 추가
add-image = 이미지 추가
all-displays = 모든 디스플레이
colors = 색상
dialog-add = 추가
fill = 채우기
fit-to-screen = 화면에 맞추기
open-new-folder = 새 폴더 열기
recent-folders = 최근 폴더

x-minutes = { $number } 분
x-hours = { $number ->
    [1] 1 시간
    *[other] { $number } 시간
}
never = 절대

## Desktop: Appearance

appearance = 모양
    .desc = 강조 색상 및 테마

accent-color = 강조 색상
app-background = 어플리케이션 또는 윈도우 배경
auto = 자동
close = 닫기
color-picker = 색상 선택기
copied-to-clipboard = 클립보드에 복사됨
copy-to-clipboard = 클립보드에 복사
dark = 어둡게
export = 내보내기
hex = 16진수
import = 가져오기
light = 밝게
mode-and-colors = 모드 및 색상
recent-colors = 최근 색상
reset-to-default = 기본값으로 재설정
rgb = RGB
window-hint-accent = 활성 윈도우 색상
window-hint-accent-toggle = 테마 강조 색상을 활성 윈도우 색상으로 사용

auto-switch = 라이드 모드와 다크 모드 간 모드 자동 전환
    .sunrise = 일출 시 라이트 모드로 전환
    .sunset = 일몰 시 다크 모드로 전환
    .next-sunrise = 다음 일출 시 라이트 모드로 전환
    .next-sunset = 다음 일몰 시 다크 모드로 전환

container-background = 컨테이너 배경
    .desc-detail = 컨테이너 배경색은 네비게이션 사이드바, 사이드 드로어, 다이얼로그와 유사한 위젯에 사용됩니다. 기본적으로 어플리케이션과 윈도우 배경에서 자동으로 파생됩니다.
    .reset = 자동으로 재설정
    .desc = 기본 컨테이너 색상은 네이베이션 사이드바, 사이드 드로어, 다이얼로그와 유사한 위젯에 사용됩니다.

control-tint = 제어 구성 요소 색조
    .desc = 표준 버튼, 검색 입력, 텍스트 입력, 그리고 유사한 구성요소의 배경으로 사용됩니다.

frosted = 시스템 인터페이스에 반투명 유리 효과 적용
    .desc = 패널, 도크, 애플릿, 런처 및 애플리케이션 라이브러리에 배경 흐림 효과를 적용합니다.

enable-export = 이 테마를 GNOME 앱에 적용합니다.
    .desc = 모든 툴킷이 자동 전환을 지원하는 것은 아닙니다. 테마를 변경한 후 비-COSMIC 앱은 다시 시작해야 할 수 있습니다.

icon-theme = 아이콘 테마
    .desc = 애플리케이션에 다른 아이콘 세트를 적용합니다.

text-tint = 인터페이스 텍스트 색조
    .desc = 다양한 표면에서 충분한 대비를 갖는 인터페이스 텍스트 색상을 도출하는 데 사용되는 색상입니다.

style = 스타일
    .round = 둥굴게
    .slightly-round = 약간 둥글게
    .square = 사각형

interface-density = 인터페이스 밀도
    .comfortable = 적당함
    .compact = 좁음
    .spacious = 넓음

window-management-appearance = 윈도우 관리
    .active-hint = 활성화 윈도우 힌트 크기
    .gaps = 타일 윈도우 주변 간격

### Experimental

experimental-settings = 실험적 설정
icons-and-toolkit = 아이콘과 툴킷 테마
interface-font = 시스템 글꼴
monospace-font = 모노스페이스 글꼴

## Desktop: Notifications

notifications = 알림
    .desc = 방해 금지 모드, 잠금 화면 알림 및 애플리케이션별 설정.

## Desktop: 패널

panel = 패널
    .desc = 데스크톱 컨트롤 및 메뉴가 있는 상단 표시줄.

add = 추카
add-applet = 애플릿 추가
all = 모두
applets = 애플릿
center-segment = 중앙 세그먼트
drop-here = 여기에 애플랏 드랍
end-segment = 세그먼트 종료
large = 크게
no-applets-found = 애플릿을 찾을 수 없음.
panel-bottom = 아래
panel-left = 왼쪽
panel-right = 오른쪽
panel-top = 위
search-applets = 애플릿 검색...
small = 작게
start-segment = 세그먼트 시작

panel-appearance = 모양
    .match = 데스트톱에 일치
    .light = 발게
    .dark = 어둡게

panel-behavior-and-position = 동작 및 위치
    .autohide = 패널 자동 숨기기
    .dock-autohide = 독 자동 숨기기
    .position = 화면에 위치
    .display = 디스플레이에 위치

panel-style = 스타일
    .anchor-gap = 패널과 화면 경계 사이에 간격
    .dock-anchor-gap = 도크와 화면 경계 사이에 간격
    .extend = 패널을 화면 경계로 확장
    .dock-extend = 도크을 화면 경계로 확장
    .appearance = 모양
    .size = 크기
    .background-opacity = 배경 투명도

panel-applets = 구성
    .dock-desc = 도크 애플릿 구성
    .desc = 패널 애플릿 구성

panel-missing = 패널 설정이 존재하지 않습니다.
    .desc = 패널 설정 파일이 사용자의 설정을 사용하는 동안 누락되었거나, 손상되었습니다.
    .fix = 기본값으로 재설정

## Desktop: Dock

dock = 도크
    .desc = 앱 트레이 및 기타 애플릿에 고정된 애플리케이션이 있는 패널입니다.

## Desktop: Window management

window-management = 윈도우 관리
    .desc = 슈퍼 키 동작, 윈도우 제어 옵션 및 추가, 윈도우 타일링 옵션.

super-key = 슈퍼 키 동작
    .launcher = 런쳐 열기
    .workspaces = 워크스페이스 열기
    .applications = 응용 프로그램 열기
    .disable = 비활성화

edge-gravity = 떠 있는 윈도우를 가까운 경계로 끌어당김

window-controls = 윈도우 컨트롤
    .maximize = 최대화 버튼 표시
    .minimize = 최소화 버튼 표시
    .active-window-hint = 활성 윈도우 힌트 표시

focus-navigation = 포커스 탐색
    .focus-follows-cursor = 포커스가 커서를 따름
    .focus-follows-cursor-delay = 포커스가 커서 지연(ms)에 따름
    .cursor-follows-focus = 커서가 포커스를 따름

## Desktop: Workspaces

workspaces = 워크스페이스
    .desc = 워크스페이스 방향 및 동작

workspaces-behavior = 워크스페이스 동작
    .dynamic = 동적 워크스페이스
    .dynamic-desc = 빈 워크스페이스를 자동으로 제거합니다.
    .fixed = 고정된 수의 워크스페이스
    .fixed-desc = 오버뷰에서 워크스페이스를 추가하거나 제거합니다.

workspaces-multi-behavior = 다중 모니터 동작
    .span = 디스플레이 확장 워크스페이스
    .separate = 디스플레이에 분할 워크스페이스가 있음

workspaces-overview-thumbnails = 워크스페이스 개요 썸네일
    .show-number = 워크스페이스 번호 보기
    .show-name = 워크스페이스 이름 보기

workspaces-orientation = 워크스페이스 방향
    .vertical = 수직
    .horizontal = 수평

hot-corner = 핫 코너
    .top-left-corner = 워크스페이스의 왼쪽 상단 핫 코너 활성화

## Displays

-requires-restart = 다시 시작 필요

color = 색상
    .depth = 색 깊이
    .profile = 색상 프로필
    .sidebar = 색상 프로필
    .temperature = 색 온도

display = 디스플레이
    .desc = 디스플레이, 그래픽 전환 및 야간 조명 관리
    .array = 디스플레이 배열
    .arrangement-desc = 디스플레이를 드래그하여 재정렬합니다.
    .enable = 디스플레이 활성화
    .external = { $size } { $output } 외부 디스플레이
    .laptop = { $size } 노트북 디스플레이
    .options = 디스플레이 옵션
    .refresh-rate = 새로 고침 빈도
    .resolution = 해상도
    .scale = 배율
    .additional-scale-options = 추가 배율 옵션

mirroring = 미러링
    .id = 미러링 { $id }
    .dont = 미러링 하지 않음
    .mirror = 미러링 { $display }
    .project = { $display ->
        [all] all displays
        *[other] { $display }
    } 로 프로젝트
    .project-count = { $count} 이외 { $count ->
        [1] display
        *[other] displays
    } 로 프로젝트

night-light = 야간 조명
    .auto = 자동 (일몰에서 일출까지)
    .desc = 청색광을 줄여 따듯한 색으로 표시

orientation = 방향
    .standard = 표준
    .rotate-90 = 90도 회전
    .rotate-180 = 180도 회전
    .rotate-270 = 270도 회전

vrr = 가변 새로 고침 빈도
    .enabled = 사용
    .force = 항상
    .auto = 자동
    .disabled = 사용 안 함

scheduling = 스케줄링
    .manual = 수동 일정

dialog = 대화 상자
    .title = 이 표시 설정을 유지하시겠습니까?
    .keep-changes = 변경 사항 유지
    .change-prompt = 설정 변경 사항은 { $time }초 후에 자동으로 되롤려집니다.
    .revert-settings = 설정 되돌리기

legacy-applications = X11 윈도우 시스템 애플리케이션 스케일링
    .scaled-by-system = 모든 X11 애플리케이션 스케일링
    .system-description = HiDPI 화면에서 X11 애플리케이션이 흐릿하게 표시됨.
    .scaled-natively = X11 애플리케이션을 기본 해상도로 렌더링합니다.
    .native-description = 스케일링을 지원하지 않는 X11 애플리케이션은 HiDPI 디스플레이를 사용할 때 작게 표시됩니다. 게임에서 전체 모니터 해상도를 활용할 수 있도록 활성화합니다.

## Sound

sound = 사운드
    .desc = N/A

sound-output = 출력
    .volume = 출력 볼륨
    .device = 출력 장치
    .level = 출력 레벨
    .config = 구성
    .balance = 잔액

sound-input = 입력
    .volume = 입력 볼륨
    .device = 입력 장치
    .level = 입력 레벨

sound-alerts = 알림
    .volume = 알림 볼륨
    .sound = 알림 소리

sound-applications = 애플리케이션
    .desc = 애플리케이션 볼륨 및 설정

profile = 프로필

## Power

power = 전원 및 배터리
    .desc = 전원 설정 관리

battery = 배터리
  .minute = { $value } { $value ->
        [one] 분
       *[other] 분
  }
  .hour = { $value } { $value ->
        [one] 시간
       *[other] 시간
  }
  .day = { $value } { $value ->
        [one] 일
       *[other] 일
  }
  .less-than-minute = 1분 미만
  .and = 그리고
  .remaining-time = { $time } 까지 { $action ->
        [full] full
       *[other] empty
   }

connected-devices = 연결된 장치
  .unknown = 알수 없는 장치

power-mode = 전원 모드
    .battery = 배터리 수명 연장
    .battery-desc = 전력 사용량 감소 및 저소음 성능.
    .balanced = 균형 잡힌
    .balanced-desc = 조용한 성능 및 적당한 전력 사용량
    .performance = 고성능
    .performance-desc = 최고 성능 및 전력 사용량
    .no-backend = 백엔드를 찾을 수 없음. system76-power 또는 power-profiles-daemon을 설치합니다.

power-saving = 절전 옵션
    .turn-off-screen-after = 이후 화면 끄기
    .auto-suspend = 자동 일시 중단
    .auto-suspend-ac = 전원이 연결되면 자동 일시 중단
    .auto-suspend-battery = 배터리 전원 시 자동 일시 중단

## Input

acceleration-desc = 속도에 따라 추적 감도를 자동으로 조정합니다.

disable-while-typing = 입력하는 동안 사용 안 함

input-devices = 입력 장치
    .desc = 입력 장치

primary-button = 기본 버튼
    .desc = 물리적 버튼의 순서를 설정합니다.
    .left = 왼쪽
    .right = 오른쪽

scrolling = 스크롤
    .two-finger = 두 손가락으로 스크롤합니다.
    .edge = 한 손가락으로 가장자리를 따라 스크롤하기
    .speed = 스크롤 속도
    .natural = 자연스러운 스크롤
    .natural-desc = 뷰 대신 콘텐츠 스크롤

## Input: Keyboard

slow = 느리게
fast = 빠르게
short = 짧게
long = 길게
keyboard = 키보드
    .desc = 입력 소스, 전환, 특수 문자 입력, 바로 가기.

keyboard-sources = 입력 소스
    .desc = 수퍼+스페이스 키 조합을 사용하여 입력 소스를 전환할 수 있습니다. 키보드 단축키 설정에서 사용자 지정할 수 있습니다.
    .move-up = 위로 이동
    .move-down = 아래로 이동
    .settings = 설정
    .view-layout = 키보드 레이아웃 보기
    .remove = 제거
    .add = 입력 소스 추가

keyboard-special-char = 특수 문자 입력
    .alternate = 대체 문자 키
    .compose = 작성 키
    .caps = 캡 잠금 키

keyboard-typing-assist = 타이핑
    .repeat-rate = 반복 속도
    .repeat-delay = 반복 지연

added = 추가됨
type-to-search = 검색할 입력...
show-extended-input-sources = 확장 입력 소스 표시

## Input: Keyboard: Shortcuts

keyboard-shortcuts = 키보드 단축키
    .desc = 바로 가기 보기 및 사용자 지정하기

add-keybinding = 키 바인딩 추가
cancel = 취소
command = 커맨드
custom = 사용자 지정
debug = 디버그
disabled = 사용 안 함
input-source-switch = 키보드 언어 입력 소스 전환
migrate-workspace-prev = 워크스페이스를 이전 출력으로 전환합니다.
migrate-workspace-next = 워크스페이스를 다음 출력으로 전환합니다.
migrate-workspace = 워크스페이스를 출력으로 전환하기 { $direction ->
    *[down] down
    [left] left
    [right] right
    [up] up
}
navigate = 탐색
replace = 바꾸기
shortcut-name = 단축키 이름
system-controls = 시스템 컨트롤
terminate = 종료
toggle-stacking = 윈도우 스태킹 전환
type-key-combination = 키 조합 입력

custom-shortcuts = 사용자 지정 바로 가기
    .add = 바로 가기 추가
    .context = 사용자 지정 바로 가기 추가
    .none = 사용자 지정 바로 가기 없음

modified = { $count } 수정됨

nav-shortcuts = 탐색
    .prev-output = 이전 출력에 포커스
    .next-output = 다음 출력에 포커스
    .last-workspace = 마지막 워크스페이스 포커스
    .prev-workspace = 이전 워크스페이스 포커스
    .next-workspace = 다음 워크스페이스 포커스
    .focus = 포커스 윈도우 { $direction ->
        *[down] down
        [in] in
        [left] left
        [out] out
        [right] right
        [up] up
    }
    .output = 출력 전환 { $direction ->
        *[down] down
        [left] left
        [right] right
        [up] up
    }
    .workspace = 워크스페이스 이동 { $num }

manage-windows = 윈도우 관리
    .close = 윈도우 닫기
    .maximize = 윈도우 최대화
    .minimize = 윈도우 최소화
    .resize-inwards = 윈도우 크기를 안쪽으로 조정합니다.
    .resize-outwards = 윈도우 크기 바깥쪽으로 조정
    .toggle-sticky = 고정 윈도우 전환

move-windows = 윈도우 이동
    .direction = 윈도우 이동 { $direction ->
        *[down] down
        [left] left
        [right] right
        [up] up
    }
    .display = 윈도우를 한 윈도우로 이동 { $direction ->
        *[down] down
        [left] left
        [right] right
        [up] up
    }
    .workspace = 윈도우를 한 워크스페이스로 이동 { $direction ->
        *[below] below
        [left] left
        [right] right
        [above] above
    }
    .workspace-num = 윈도우를 워크스페이스로 이동 { $num }
    .prev-workspace = 윈도우를 이전 워크스페이스로 이동
    .next-workspace = 윈도우를 다음 워크스페이스로 이동
    .last-workspace = 윈도우를 마지막 워크스페이스로 이동
    .next-display = 윈도우를 다음 디스플레이로 이동
    .prev-display = 윈도우를 이전 디스플레이로 이동
    .send-to-prev-workspace = 윈도우를 이전 워크스페이스로 이동
    .send-to-next-workspace = 윈도우를 다음 워크스페이스로 이동

system-shortcut = 시스템
    .app-library = 앱 라이브러리 열기
    .brightness-down = 디스플레이 밝기 낮추기
    .brightness-up = 디스플레이 밝기 증가
    .home-folder = 홈 폴더 열기
    .keyboard-brightness-down = 키보드 밝기 감소
    .keyboard-brightness-up = 키보드 밝기 높이기
    .launcher = 런처 열기
    .log-out = 로그아웃
    .lock-screen = 화면 잠금
    .mute = 오디오 출력 음소거
    .mute-mic = 마이크 입력 음소거
    .play-pause = 재생/일시 정지
    .play-next = 다음 트랙
    .play-prev = 이전 트랙
    .screenshot = 스크린샷 찍기
    .terminal = 터미널 열기ㄲ
    .volume-lower = 오디오 출력 볼륨 줄이기
    .volume-raise = 오디오 출력 볼륨 높이기
    .web-browser = 웹 브라우저 열기
    .window-switcher = 열려 있는 윈도우 사이를 전환합니다.
    .window-switcher-previous = 열린 윈도우 사이를 반대로 전환합니다.
    .workspace-overview = 워크스페이스 개요 열기

window-tiling = 윈도우 타일링
    .horizontal = 가로 방향 설정
    .vertical = 세로 방향 설정
    .swap-window = 윈도우 바꾸기
    .toggle-tiling = 윈도우 바둑판식 타일링 전환
    .toggle-stacking = 윈도우 스태킹 전환 = 윈도우 스태킹 전환
    .toggle-floating = 윈도우 플로팅 토글
    .toggle-orientation = 방향 전환 = 방향 전환

replace-shortcut-dialog = 바로가기를 바꾸시겠습니까?
    .desc = { $shortcut }이 { $name }에서 사용됩니다. 변경하면 { $name }이 비활성화됩니다.

zoom-in = 줌인
zoom-out = 줌아웃

## Input: Mouse

mouse = 마우스
    .desc = 마우스 속도, 가속, 자연스러운 스크롤.
    .speed = 마우스 속도
    .acceleration = 마우스 가속 사용

## Input: Touchpad

click-behavior = 클릭 동작
    .click-finger = 두 손가락으로 보조 클릭, 세 손가락으로 가운데 클릭
    .button-areas = 오른쪽 하단 모서리에서 보조 클릭, 가운데 하단에서 가운데 클릭

pinch-to-zoom = 핀치하여 줌
    .desc = Zoom을 지원하는 어플리케이션의 경우, 두 손가락을 사용하여 컨텐츠를 확대합니다.

tap-to-click = 탭하여 클릭
    .desc = 한손가락으로 탭하면 기본 클릭, 두손가락으로 탭하면 보조 클릭, 세손가락으로 탭하면 중간 클릭을 사용할 수 있습니다.

touchpad = Touchpad
    .acceleration = 터치패드 가속 활성화
    .desc = 터치패드 속도, 클릭 옵션, 제스처
    .speed = 터치패드 속도

## Input: 제스쳐

gestures = 제스쳐
    .four-finger-down = 네 손가락으로 아래로 스와이프
    .four-finger-left = 네 손가락으로 왼쪽으로 스와이프
    .four-finger-right = 네 손가락으로 오른쪽으로 스와이프
    .four-finger-up = 네 손가락으로 위로 스와이프
    .three-finger-any = 세 손가락으로 아무 방향이나 스와이프

switch-workspaces = 워크스페이스 전환
    .horizontal = 네 손가락으로 오른쪽/왼쪽 스와이프
    .vertical = 네 손가락으로 위/아래 스와이프

switch-between-windows = 윈도우간 전환
open-application-library = 응용프로그램 라이브러리 열기
open-workspaces-view = 워크스페이스 Overview 열기

## Time & Language

time = 시간 & 언어
    .desc = N/A

time-date = 날짜 & 시간
    .desc = 표준 시간대, 자동 시계 설정 및 시간 표현 형태.
    .auto = 자동 설정
    .auto-ntp = 날짜 & 시간은 표준 시간대가 설정되면 자동으로 갱신됩니다.

time-zone = 표준 시간대
    .auto = 표준 시간대 자동 설정
    .auto-info = 지역 기반 서비스와 인터넷 접근 요청

time-format = 날짜 및 시간 형식
    .twenty-four = 24시간 
    .show-seconds = 초 표시
    .first = 첫번째 요일 표시
    .show-date = 상단 패널에 날짜 표시
    .friday = 금요일
    .saturday = 토요일
    .sunday = 일요일
    .monday = 월요일

time-region = 지역 및 언어
    .desc = 지역에 따른 날짜, 시간 및 숫자 서식 지정

formatting = 서식 지정
    .dates = 날짜
    .time = 시간
    .date-and-time = 날짜 & 시간
    .numbers = 숫자
    .measurement = 측정
    .paper = 페이지

preferred-languages = 선호 언어
    .desc = 언어 순서에 따라 데스크톱 번역에 사용되는 언어가 결정됩니다. 변경 사항은 다음 로그인 시 적용됩니다.

add-language = 언어 추가
    .context = 언어 추가
install-additional-languages = 추가 언어 설치
region = 지역

## System

system = 시스템 & 계정

## System: About

about = About
    .desc = 장치명, 하드웨어 정보, 운영 체계 기본

about-device = 장치명
    .desc = 이 명칭은 다른 네트워크나 블루투스 장치에 나타납니다.

about-hardware = 하드웨어
    .model = 하드웨어 모델
    .memory = 메모리
    .processor = 프로세서
    .graphics = 그래픽
    .disk-capacity = 디스크 용량

about-os = 운영 체계
    .os = 운영 체계
    .os-architecture = 운영 체계 구조
    .desktop-environment = 데스트톱 환경
    .windowing-system = 윈도우 시스템

about-related = 관련 설정
    .support = 지원 받기

## System: Firmware

firmware = 펌웨어
    .desc = 펌웨어 상세.

## System: Users

users = Users
    .desc = 인증과 사용자 계정.
    .admin = 관리
    .standard = 일반
    .profile-add = 프로파일 이미지 선택

administrator = 관리자
    .desc = 관리자는 모든 사용자의 설정을 수정하거나 다른 사용자를 추가 제거 할수 있습니다.

add-user = 사용자 추가
remove-user = 사용자 삭제
full-name = 이름

## System: Default Applications

default-apps = 기본 어플리케이션
    .desc = 기본 웹 브라우저, 메일 클라이언트, 파일 관리자 및 기타 어플리케이션
    .web-browser = 웹 브라우저
    .file-manager = 파일 관리자
    .mail-client = 메일 클라이언트
    .music = 음악
    .video = 비디오
    .photos = 사진
    .calendar = 캘린더
    .terminal = 터미널
    .other-associations = 기타 연결
    .text-editor = 문서 편집기
