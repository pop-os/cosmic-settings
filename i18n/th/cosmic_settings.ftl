app = การตั้งค่า COSMIC

dbus-connection-error = ล้มเหลวในการเชื่อมต่อไปยัง DBus
ok = OK
unknown = ไม่ทราบ

number = { $number }

## Network & Wireless

connections-and-profiles = การเชื่อมต่อ{ $variant ->
    [wired] แบบมีสาย
    [wifi] ไวไฟ
    [vpn] VPN
    *[other] ที่ไม่รู้จัก
} และโปรไฟล์การเชื่อมต่อ

add-network = เพิ่มเครือข่าย
    .profile = เพิ่มโปรไฟล์
add-vpn = เพิ่ม VPN
airplane-on = โหมดเครื่องบินเปิดอยู่
cable-unplugged = สายถูกถอด
connect = เชื่อมต่อ
connected = เชื่อมต่อแล้ว
connecting = กำลังเชื่อมต่อ…
disconnect = ตัดการเชื่อมต่อ
forget = ลืม
known-networks = เครือข่ายที่รู้จัก
network-and-wireless = เครือข่ายและไร้สาย
no-networks = ไม่พบเจอเครือข่าย
no-vpn = ไม่มีเครือข่าย VPN
password = รหัสผ่าน
remove = ลบ
settings = การตั้งค่า
username = ชื่อผู้ใช้
visible-networks = เครือข่ายที่มองเห็น

auth-dialog = จำเป็นต้องเข้าสู่ระบบ
    .vpn-description = ใส่ชื่อผู้ใช้และรหัสผ่านที่จำเป็นต่อบริการ VPN
    .wifi-description = ใส่รหัสผ่านหรือกุญแจเข้ารหัส คุณสามารถเชื่อมต่อโดยใช้ปุ่ม "WPS" บนเราเตอร์ได้เช่นกัน

forget-dialog = ลิมเครือข่ายไวไฟนี้หรือไม่
    .description = คุณจะต้องใส่รหัสผ่านอีกครั้งในการเชื่อมต่อเครือข่ายไวไฟนี้ในอนาคต

network-device-state =
    .activated = เชื่อมต่อแล้ว
    .config = กำลังเชื่อมต่อ
    .deactivating = กำลังตัดการเชื่อมต่อ
    .disconnected = ตัดการเชื่อมต่อแล้ว
    .failed = ล้มเหลวในการเชื่อมต่อ
    .ip-check = กำลังตรวจสอบการเชื่อมต่อ
    .ip-config = กำลังขอ IP และข้อมูลเส้นทางเครือข่าย
    .need-auth = ต้องการการเข้าสู่ระบบ
    .prepare = กำลังเตรียมพร้อมในการเชื่อมต่อ
    .secondaries = กำลังรอการเชื่อมต่อรอง
    .unavailable = ไม่มีมีอยู่
    .unknown = ไม่ทราบสถานะ
    .unmanaged = ไม่ได้รับการจัดการ
    .unplugged = สายถูกถอด

remove-connection-dialog = นำโปรไฟล์การเชื่อมต่อออกหรือไม่
    .vpn-description = คุณจำเป็นต้องใส่รหัสผ่านอีกครั้งในการใช้เครือข่ายนี้อีกในอนาคต
    .wired-description = คุณจะต้องสร้างโปรไฟล์นี้ซ้ำในการจะใช้มันอีกในอนาคต

vpn = VPN
    .connections = การเชื่อมต่อ VPN
    .error = ล้มเหลวในการเพิ่มการตั้งค่า VPN
    .remove = ลบโปรไฟล์การเชื่อมต่อ
    .select-file = เลือกไฟล์การตั้งค่า VPN

vpn-error = ความล้มเหลวที่เกี่ยวกับ VPN
    .config = ล้มเหลวในการเพิ่มการตั้งค่า VPN
    .connect = ล้มเหลวในการเชื่อมต่อไปยัง VPN
    .connection-editor = ตัวแก้ไขการเชื่อมต่อล้มเหลว
    .connection-settings = ล้มเหลวในการรับการตั้งค่าสำหรับการเชื่อมต่อที่เปิดอยู่
    .updating-state = ล้มเหลวในการอัพเดทสถานะตัวจัดการเครือข่าย
    .wireguard-config-path = เส้นทางไฟล์ของไฟล์การตั้งค่า WireGuard ไม่ถูกต้อง
    .wireguard-config-path-desc = ไฟล์ที่เลือกจะต้องอยู่บนระบบไฟล์ภายในเครื่อง
    .wireguard-device = ล้มเหลวในการสร้างอุปกรณ์ WireGuard
    .with-password = ล้มเหลวในการตั้ง{ $field ->
        *[username] ชื่อผู้ใช้
        [password] รหัสผ่าน
        [password-flags] password-flags
    } VPN ด้วย nmcli

wired = การเชื่อมต่อแบบมีสาย
    .adapter = อะแดปเตอร์การเชื่อมต่อ { $id }
    .connections = การเชื่อมต่อแบบมีสาย
    .devices = อุปกรณ์มีสาย
    .remove = ลบโปรไฟล์การเชื่อมต่อ

wifi = Wi-Fi
    .adapter = อะแดปเตอร์ Wi-Fi { $id }
    .forget = ลืมเครือข่ายนี้

wireguard-dialog = เพิ่มอุปกรณ์ WireGuard
    .description = เลือกชื่ออุปกรณ์สำหรับการตั้งค่า WireGuard

## Networking: Online Accounts

online-accounts = บัญชีออนไลน์
    .desc = เพิ่มบัญชีต่าง ๆ อีเมล IMAP และ SMTP และการล็อกอินของบริษัท

# Bluetooth

confirm = ยืนยัน

bluetooth = บลูทูธ
    .desc = จัดการอุปกรณ์บลูทูธ
    .status = อุปกรณ์นี้สามารถมองเห็นได้ด้วยชื่อ { $aliases } ขณะที่การตั้งค่าบลูทูธเปิดอยู่
    .connected = เชื่อมต่อแล้ว
    .connecting = กำลังเชื่อมต่อ
    .disconnecting = กำลังตัดการเชื่อมต่อ
    .connect = เชื่อมต่อ
    .disconnect = ตัดการเชื่อมต่อ
    .forget = ลืม
    .dbus-error = เกิดข้อผิดพลาดระหว่างการติดต่อกับ DBus: { $why }

bluetooth-paired = อุปกรณ์ที่เชื่อมต่อก่อนหน้านี้
    .connect = เชื่อมต่อ
    .battery = แบตเตอรี่ { $percentage }%

bluetooth-confirm-pin = ยืนยันรหัส PIN บลูทูธ
    .description = โปรดยืนยันว่ารหัส PIN ต่อไปนี้ตรงกับรหัสที่แสดงอยู่บน { $device }

bluetooth-available = อุปกรณ์ใกล้เคียง

bluetooth-adapters = อะแดปเตอร์บลูทูธ

## Desktop

desktop = เดสก์ทอป

## Desktop: Wallpaper

wallpaper = วอลเปเปอร์
    .change = เปลี่ยนภาพทุก ๆ
    .desc = ภาพวอลเปเปอร์ สี และตัวเลือกสไลด์โชว์
    .fit = การทำให้วอลเปเปอร์พอดี
    .folder-dialog = เลือกโฟลเดอร์วอลเปเปอร์
    .image-dialog = เลือกรูปภาพวอลเปเปอร์
    .plural = วอลเปเปอร์
    .same = วอลเปเปอร์เดียวกันบนทุกหน้าจอ
    .slide = สไดล์โชว์

add-color = เพิ่มสี
add-image = เพิ่มรูปภาพ
all-displays = หน้าจอทั้งหมด
colors = สี
dialog-add = เพิ่ม
fill = ขยายให้เต็ม
fit-to-screen = พอดีกับหน้าจอ
open-new-folder = เปิดโฟลเดอร์ใหม่
recent-folders = โฟลเดอร์ล่าสุด

x-minutes = { $number } นาที
x-hours = { $number } ชั่วโมง
never = ไม่เคย

## Desktop: Appearance

appearance = หน้าตา
    .desc = ธีมและสีหลัก

accent-color = สีหลัก
app-background = พื้นหลังแอพลิเคชั่นหรือหน้าต่าง
auto = อัตโนมัติ
close = ปิด
color-picker = เครื่องมือเลือกสี
copied-to-clipboard = คัดลอกไปยังคลิปบอร์ดแล้ว
copy-to-clipboard = คัดลอกไปยังคลิปบอร์ด
dark = มืด
export = ส่งออก
hex = Hex
import = นำเข้า
light = สว่าง
mode-and-colors = โหมดและสี
recent-colors = สีล่าสุด
reset-to-default = รีเซ็ตเป็นค่าเริ่มต้น
rgb = RGB
window-hint-accent = สีที่บอกใบ้ว่าหน้าต่างที่ใช้งานอยู่
window-hint-accent-toggle = ใช้สีหลักธีมเป็นสีที่บอกใบ้หน้าต่างที่ใช้งานอยู่

auto-switch = สลับระหว่างโหมดมืดและสว่างอัตโนมัติ
    .sunrise = สลับเป็นโหมดสว่างเมื่อพระอาทิตย์ขึ้น
    .sunset = สลับเป็นโหมดมืดเมื่อพระอาทิตย์ตก
    .next-sunrise = สลับเป็นโหมดสว่างเมื่อพระอาทิตย์ขึ้นครั้งถัดไป
    .next-sunset = สลับเป็นโหมดมืดเมื่อพระอาทิตย์ตกครั้งถัดไป

container-background = พื้นหลังคอนเทนเนอร์
    .desc-detail = สีคอนเทนเนอร์จะถูกใช้สำหรับแถบนำทางด้านข้าง ลิ้นชักข้าง ไดอะล็อก และวิดเจ็ตที่คล้ายกัน โดยค่าเริ่มต้น สีนี้จะได้มาจากพื้นหลังแอพลิเคชั่นหรือหน้าต่าง
    .reset = รีเซ็ตเป็นอัตโนมัติ
    .desc = สีหลักคอนเทนเนอร์จะถูกใช้สำหรับแถบนำทางด้านข้าง ลิ้นชักข้าง ไดอะล็อก และวิดเจ็ตที่คล้ายกัน

control-tint = สีส่วนประกอบควบคุม
    .desc = ใช้สำหรับพื้นหลังของปุ่มมาตรฐาน ช่องค้นหา ช่องใส่ข้อความ และส่วนประกอบที่คล้ายกัน

frosted = เอฟเฟกต์กระจกฝ้าบนอินเทอร์เฟซระบบ
    .desc = เพิ่มการเบลอพื้นหลังให้กับแผงควบคุม แท่นวาง แอพเล็ต ตัวเรียกใช้งาน และไลบรารีแอปพลิเคชัน

enable-export = นำธีมนี้ไปใช้กับแอพ GNOME
    .desc = ชุดเครื่องมือไม่รองรับการสลับอัตโนมัติทั้งหมด อาจต้องรีสตาร์ทแอพลิเคชั่นที่ไม่ใช่ COSMIC หลังจากการเปลี่ยนธีม

icon-theme = ธีมไอคอน
    .desc = นำเซ็ตไอคอนอื่นไปใช้กับแอพลิเคชั่น

text-tint = สีข้อความอินเทอร์เฟซ
    .desc = สีที่ใช้เพื่อให้ได้สีข้อความอินเทอร์เฟซที่มีความคมชัดเพียงพอบนพื้นผิวต่างๆ

style = สไตล์
    .round = กลม
    .slightly-round = กลมเล็กน้อย
    .square = เหลี่ยม

interface-density = ความหนาแน่นของอินเทอร์เฟซ
    .comfortable = สะบาย
    .compact = กะทัดรัด
    .spacious = กว้างขวาง

window-management-appearance = การจัดการหน้าต่าง
    .active-hint = ขนาดคำใบ้หน้าต่างที่ถูกใช้งาน
    .gaps = พื้นที่ว่างระหว่างหน้าต่างเมื่อวางติดกัน

### Experimental

experimental-settings = การตั้งค่าทดลอง
icons-and-toolkit = ไอคอนและธีมชุดเครื่องมือ
interface-font = ฟอนต์ระบบ
monospace-font = ฟอนต์โมโนสเปซ

## Desktop: Notifications

notifications = การแจ้งเตือน
    .desc = ห้ามรบกวน การแจ้งเตือนบนหน้าจอล็อค และการตั้งค่าแต่ละแอพลิเคชั่น

## Desktop: Panel

panel = แผงควบคุม
    .desc = แถบด้านบนที่มีการควบคุมเดสก์ทอปและเมนู

add = เพิ่ม
add-applet = เพิ่มแอพเล็ต
all = ทั้งหมด
applets = แอพเล็ต
center-segment = ส่วนตรงกลาง
drop-here = วางแอพเล็ตที่นี่
end-segment = สิ้นสุดส่วนนี้
large = ใหญ่
no-applets-found = ไม่พบแอพเล็ต...
panel-bottom = ล่าง
panel-left = ซ้าย
panel-right = ขวา
panel-top = บน
search-applets = ค้นหาแอพเล็ต...
small = เล็ก
start-segment = เริ่มส่วนใหม่

panel-appearance = หน้าตา
    .match = ตรงกับเดสก์ทอป
    .light = สว่าง
    .dark = มืด

panel-behavior-and-position = พฤติกรรมและตำแหน่ง
    .autohide = ซ่อนแผงควบคุมโดยอัติโนมัติ
    .dock-autohide = ซ่อนแท่นวางโดยอัติโนมัติ
    .position = ตำแหน่งบนหน้าจอ
    .display = แสดงบนหน้าจอ

panel-style = สไตล์
    .anchor-gap = ช่องว่างระหว่างแผงควบคุมและขอบหน้าจอ
    .dock-anchor-gap = ช่องว่างระหว่างแท่นวางและขอบหน้าจอ
    .extend = ขยายแผงควบคุมไปยังขอบหน้าจอ
    .dock-extend = ขยายแท่นวางไปยังขอบหน้าจอ
    .appearance = หน้าตา
    .size = ขนาด
    .background-opacity = ความทึบแสงพื้นหลัง

panel-applets = ตั้งค่า
    .dock-desc = ตั้งค่าแอพเล็ทบนแท่นวาง
    .desc = ตั้งค่าแอพเล็ทบนแผงควบคุม

panel-missing = การตั้งค่าแผงควบคุมสูญหาย
    .desc = ไฟล์การตั้งค่าแผงควบคุมสูญหายเนื่องจากการใช้การตั้งค่าแบบกำหนดเองหรือไฟล์เกิดการปนเปื้อน
    .fix = รีเซ็ตเป็นค่าเริ่มต้น

## Desktop: Dock

dock = แท่นวาง
    .desc = แผงที่มีแอพลิเคชั่นปักหมุดในถาดแอพและแอพเล็ตอื่น ๆ

## Desktop: Window management

window-management = การจัดการหน้าต่าง
    .desc = การกระทำซุปเปอร์คีย์ ตัวเลือกการควบคุมหน้าต่าง และตัวเลือกการไทล์หน้าต่างเพิ่มเติม

super-key = ซุปเปอร์คีย์
    .launcher = เปิดลันเชอร์
    .workspaces = เปิดหน้าพื้นที่ทำงาน
    .applications = เปิดหน้าแอพลิเคชั่น
    .disable = ปิด

window-controls = การควบคุมหน้าต่าง
    .minimize = แสดงปุ่มซ่อนหน้าต่าง
    .maximize = แสดงปุ่มขยายหน้าต่างให้ใหญ่ที่สุด
    .active-window-hint = แสดง

focus-navigation = Focus Navigation
    .focus-follows-cursor = Focus follows cursor
    .focus-follows-cursor-delay = Focus follows cursor delay in ms
    .cursor-follows-focus = Cursor follows focus

## Desktop: Workspaces

workspaces = พื้นที่ทำงาน
    .desc = พฤติกรรมและการวางแนวพื้นที่ทำงาน

workspaces-behavior = พฤติกรรมพื้นที่ทำงาน
    .dynamic = พื้นที่ทำงานที่เปลี่ยนแปลงอยู่เสมอ
    .dynamic-desc = ลบที่ทำงานว่างปล่าวโดยอัตโนมัติ
    .fixed = จำนวนพื้นที่ทำงานคงที่
    .fixed-desc = เพิ่มหรือลบพื้นที่ทำงานในหน้าภาพรวม

workspaces-multi-behavior = พฤติกรรมหลายหน้าจอ
    .span = พื้นที่ทำงานครอบคลุมหลายหน้าจอ
    .separate = แต่ละหน้าจอมีพื้นที่ทำงานที่แยกกัน

workspaces-overview-thumbnails = ภาพย่อภาพรวมพื้นที่ทำงาน
    .show-number = แสดงหมายเลขพื้นที่ทำงาน
    .show-name = แสดงชื่อพื้นที่ทำงาน

workspaces-orientation = การวางแนวพื้นที่ทำงาน
    .vertical = แนวตั้ง
    .horizontal = แนวนอน

hot-corner = มุมด่วน
    .top-left-corner = เปิดมุมด่วนบริเวณมุมบนซ้ายสำหรับพื้นที่ทำงาน

## Displays

-requires-restart = จำเป็นต้องรีสตาร์ท

color = สี
    .depth = ความลึกของสี
    .profile = โปรไฟล์สี
    .sidebar = โปรไฟล์สี
    .temperature = อุณหภูมิสี

display = จอแสดงผล
    .desc = จัดการจอแสดงผล การสลับกราฟิก และไฟกลางคืน
    .arrangement = การจัดจอแสดงผล
    .arrangement-desc = ลากจอแสดงผลเพื่อจัดเรียงพวกมัน
    .enable = เปิดจอแสดงผล
    .external = { $size } { $output } จอแสดงผลภายนอก
    .laptop = { $size } จอแสดงผลแล็ปท็อป
    .options = การตั้งค่าจอแสดงผล
    .refresh-rate = รีเฟรชเรท
    .resolution = ความละเอียด
    .scale = ขนาด

mirroring = การสะท้อน
    .id = การสะท้อน { $id }
    .dont = อย่าสะท้อน
    .mirror = สะท้อน { $display }
    .project = ฉายภาพไปยัง { $display ->
        [all] จอภาพทั้งหมด
        *[other] { $display }
    }
    .project-count = กำลังฉายไปยังจอภาพอื่นอีก { $count } จอ

night-light = ไฟกลางคืน
    .auto = อัตโนมัติ (ช่วงพระอาทิตย์ตกถึงพระอาทิตย์ขึ้น)
    .desc = ลดแสงสีฟ้าด้วยสีที่อุ่นขึ้น

orientation = การวางแนว
    .standard = มาตรฐาน
    .rotate-90 = หมุน 90
    .rotate-180 = หมุน 180
    .rotate-270 = หมุน 270

vrr = อัตราการรีเฟรชแบบแปรผัน
    .enabled = เปิด
    .force = บังคับเปิด
    .auto = อัตโนมัติ
    .disabled = ปิด

scheduling = การจัดกำหนดการ
    .manual = การจัดกำหนดการด้วยมือ

dialog = ไดอะล็อก
    .title = เก็บการตั้งค่าหน้าจอเหล่านี้ไว้หรือไม่
    .keep-changes = เก็บการเปลี่ยนแปลงไว้
    .change-prompt = การเปลี่ยนแปลงการตั้งค่าจะคืนค่าเดิมภายใน { $time } วินาที
    .revert-settings = คงค่าการตั้งค่าเดิม

legacy-applications = การขยายแอพลิเคชั่นที่ใช้ระบบจัดการหน้าต่าง X11
    .scaled-by-system = ขยายแอพลิเคชั่น X11 ทั้งหมด
    .system-description = แอพลิเคชั่น X11 จะเบลอบนหน้าจอ HiDPI
    .scaled-natively = แสดงแอพลิเคชั่น X11 ทั้งหมดที่ความละเอียดดั้งเดิม
    .native-description = แอพลิเคชั่น X11 ที่ไม่รองรับการขยายจะเล็กเมื่อใช้หน้าจอ HiDPI เปิดสำหรับเกมเพื่อใช้ประโยชน์ความละเอียดจอทั้งหมด

## Sound

sound = เสียง
    .desc = N/A

sound-output = เอาท์พุต
    .volume = ระดับเสียงเอาท์พุต
    .device = อุปกรณ์เอาท์พุต
    .level = ระดับเอาท์พุต
    .config = การตั้งค่า
    .balance = สมดุล

sound-input = อินพุต
    .volume = ระดับเสียงอินพุต
    .device = อุปกรณ์อินพุต
    .level = ระดับอินพุต

sound-alerts = การแจ้งเตือน
    .volume = ระดับเสียงการแจ้งเตือน
    .sound = เสียงการแจ้งเตือน

sound-applications = แอพลิเคชั่น
    .desc = การตั้งค่าและระดับเสียงแอพลิเคชั่น

profile = โปรไฟล์

## Power

power = พลังงานและแบตเตอรี่
    .desc = จัดการการตั้งค่าพลังงาน

battery = แบตเตอรี่
    .minute = { $value } นาที
  .hour = { $value } ชั่วโมง
  .day = { $value } วัน
  .less-than-minute = น้อยกว่าหนึ่งนาที
  .and = และ
  .remaining-time = { $time } ถึงจะ{ $action ->
        [full] เต็ม
       *[other] หมด
   }

connected-devices = อุปกรณ์ที่เชื่อมต่อ
    .unknown = อุปกรณ์ที่ไม่รู้จัก

power-mode = โหมดพลังงาน
    .battery = ยืดเวลาแบตเตอรี่
    .battery-desc = การใช้พลังงานที่ลดลงและประสิทธิภาพที่ไร้เสียงรบกวน
    .balanced = สมดุล
    .balanced-desc = การใช้พลังงานปานกลางและประสิทธิภาพที่ลดเสียงรบกวนลง
    .performance = ประสิทธิภาพสูง
    .performance-desc = ประสิทธิภาพและการใช้พลังงานสูงสุด
    .no-backend = ไม่พบแบ็คเอนด์ กรุณาติดตั้ง system76-power หรือ power-profiles-daemon

power-saving = ตัวเลือกประหยัดพลังงาน
    .turn-off-screen-after = ปิดหน้าจอหลังจาก
    .auto-suspend = ปิดเครื่องโดยอัตโนมัติ
    .auto-suspend-ac = ปิดเครื่องโดยอัตโนมัติเมื่อเสียบปลั๊กอยู่
    .auto-suspend-battery = ปิดเครื่องโดยอัตโนมัติเมื่อใช้พลังงานแบตเตอรี่อยู่

## Input

acceleration-desc = ปรับความไวในการติดตามโดยอัตโนมัติตามความเร็ว

disable-while-typing = ปิดการใช้งานระหว่างการพิมพ์

input-devices = อุปกรณ์ป้อนข้อมูล
    .desc = อุปกรณ์ป้อนข้อมูล

primary-button = ปุ่มหลัก
    .desc = ตั้งลำดับปุ่มกายภาพ
    .left = ซ้าย
    .right = ขวา

scrolling = การเลื่อน
    .two-finger = เลื่อนด้วยสองนิ้ว
    .edge = เลื่อนตามขอบด้วยหนึ่งนิ้ว
    .speed = ความเร็วการเลื่อน
    .natural = การเลื่อนอย่างเป็นธรรมชาติ
    .natural-desc = เลื่อนเนื้อหาแทนมุมมอง

## Input: Keyboard

slow = ช้า
fast = เร็ว
short = สั้น
long = ยาว
keyboard = คีย์บอร์ด
    .desc = แหล่งอินพุต การสลับ การพิมพ์ตัวอักษรพิเศษ และคีย์ลัด

keyboard-sources = แหล่งอินพุต
    .desc = สามารถสลับแหล่งอินพุตได้โดยคีย์ ซุปเปอร์ + เว้นวรรค ปุ่มนี้สามารถปรับแต่งได้ผ่านการตั้งค่าคีย์ลัด
    .move-up = ขยับขึ้น
    .move-down = ขยับลง
    .settings = การตั้งค่า
    .view-layout = ดูแผนผังคีย์บอร์ด
    .remove = ลบ
    .add = เพิ่มแหล่งอินพุต

keyboard-special-char = การพิมพ์ตัวอักษรพิเศษ
    .alternate = คีย์อักขระอื่น
    .compose = คีย์เรียบเรียง
    .caps = ปุ่ม Caps Lock

keyboard-typing-assist = การพิมพ์
    .repeat-rate = อัตราการทำซ้ำ
    .repeat-delay = ดีเลย์การทำซ้ำ

added = เพิ่มแล้ว
type-to-search = พิมพ์เพื่อค้นหา
show-extended-input-sources = แสดงแหล่งอินพุตเพิ่มเติม

## Input: Keyboard: Shortcuts

keyboard-shortcuts = คีย์ลัด
    .desc = ดูและปรับแต่งคีย์ลัด

add-keybinding = เพิ่มปุ่มลัด
cancel = ยกเลิก
command = คำสั่ง
custom = กำหนดเอง
debug = ดีบัก
disabled = ปิดใช้งาน
migrate-workspace-prev = ย้ายพื้นที่ทำงานไปยังเอาท์พุตก่อนหน้า
migrate-workspace-next = ย้ายพื้นที่ทำงานไปยังเอาท์พุตถัดไป
migrate-workspace = ย้ายพื้นที่ทำงานไปยังเอาท์พุต{ $direction ->
    *[down] ล่าง
    [left] ทางซ้าย
    [right] ทางขวา
    [up] บน
}
navigate = การนำทาง
replace = แทนที่
shortcut-name = ชื่อคีย์ลัด
system-controls = การควบคุมระบบ
terminate = ยุติ
toggle-stacking = สลับการซ้อนหน้าต่าง
type-key-combination = พิมพ์คีย์ของคีย์ลัด

custom-shortcuts = คีย์ลัดกำหนดเอง
    .add = เพิ่มคีย์ลัด
    .context = เพิ่มคีย์ลัดกำหนดเอง
    .none = ไม่มีคีย์ลัดกำหนดเอง

modified = { $count } รายการถูกแก้ไข

nav-shortcuts = การนำทาง
    .prev-output = โฟกัสเอาท์พุตก่อนหน้า
    .next-output = โฟกัสเอาท์พุตถัดไป
    .last-workspace = โฟกัสพื้นที่ทำงานสุดท้าย
    .prev-workspace = โฟกัสพื้นที่ทำงานก่อนหน้า
    .next-workspace = โฟกัสพื้นที่ทำงานถัดไป
    .focus = โฟกัสหน้าต่าง{ $direction ->
        *[down] ด้านล่าง
        [in] ใน
        [left] ด้านซ้าย
        [out] นอก
        [right] ด้านขวา
        [up] ด้านบน
    }
    .output = สลับไปยังเอาท์พุต{ $direction ->
        *[down] ด้านล่าง
        [left] ด้านซ้าย
        [right] ด้านขวา
        [up] ด้านบน
    }
    .workspace = สลับไปยังพื้นที่ทำงาน { $num }

manage-windows = จัดการหน้าต่าง
    .close = ปิดหน้าต่าง
    .maximize = ขยายหน้าต่าง
    .minimize = ซ่อนหน้าต่าง
    .resize-inwards = ปรับขนาดหน้าต่างเข้า
    .resize-outwards = ปรับขนาดหน้าต่างออก
    .toggle-sticky = ปิดเปิดหน้าต่างเหนียว

move-windows = ขยับหน้าต่าง
    .direction = ขยับหน้าต่าง{ $direction ->
        *[down] ลง
        [left] ไปทางซ้าย
        [right] ไปทางขวา
        [up] ขึ้น
    }
    .display = ขยับหน้าต่างไปยังจอแสดงผล{ $direction ->
        *[down] ด้านล่าง
        [left] ทางซ้าย
        [right] ทางขวา
        [up] ด้านบน
    }
    .workspace = ขยับหน้าต่างไปยังพื้นที่ทำงาน{ $direction ->
        *[below] ด้านล่าง
        [left] ทางซ้าย
        [right] ทางขวา
        [above] ด้านบน
    }
    .workspace-num = ขยับหน้าต่างไปยังพื้นที่ทำงาน { $num }
    .prev-workspace = ขยับหน้าต่างไปยังพื้นที่ทำงานก่อนหน้า
    .next-workspace = ขยับหน้าต่างไปยังพื้นที่ทำงานถัดไป
    .last-workspace = ขยับหน้าต่างไปยังพื้นที่ทำงานสุดท้าย
    .next-display = ขยับหน้าต่างไปยังจอแสดงผลถัดไป
    .prev-display = ขยับหน้าต่างไปยังจอแสดงผลก่อนหน้า
    .send-to-prev-workspace = ขยับหน้าต่างไปยังพื้นที่ทำงานก่อนหน้า
    .send-to-next-workspace = ขยับหน้าต่างไปยังพื้นที่ทำงานถัดไป

system-shortcut = ระบบ
    .app-library = เปิดไลบรารีแอพ
    .brightness-down = ลดความสว่างจอแสดงผล
    .brightness-up = เพิ่มความสว่างจอแสดงผล
    .home-folder = เปิดโฟลเดอร์บ้าน
    .keyboard-brightness-down = ลดความสว่างคีย์บอร์ด
    .keyboard-brightness-up = เพิ่มความสว่างคีย์บอร์ด
    .launcher = เปิดลันเชอร์
    .log-out = ล็อกเอาท์
    .lock-screen = ล็อคหน้าจอ
    .mute = ปิดเสียงเอาท์พุต
    .mute-mic = ปิดเสียงอินพุตไมโครโฟน
    .play-pause = เล่น/หยุด
    .play-next = เพลงถัดไป
    .play-prev = เพลงก่อนหน้า
    .screenshot = ถ่ายภาพหน้าจอ
    .terminal = เปิดเทอร์มินัล
    .volume-lower = ลดระดับเสียงเอาท์พุต
    .volume-raise = เพิ่มระดับเสียงเอาท์พุต
    .web-browser = เปิดเบราเซอร์
    .window-switcher = สลับระหว่างหน้าต่างที่เปิดอยู่
    .window-switcher-previous = สลับไปยังหน้าต่างก่อนหน้า
    .workspace-overview = เปิดภาพรวมพื้นที่ทำงาน

window-tiling = การไทล์หน้าต่าง
    .horizontal = ตั้งวางแนวเป็นแนวนอน
    .vertical = ตั้งการวางแนวเป็นแนวตั้ง
    .swap-window = สลับหน้าต่าง
    .toggle-tiling = ปิดเปิดการไทล์หน้าต่าง
    .toggle-stacking = ปิดเปิดการซ้อนหน้าต่าง
    .toggle-floating = ปิดเปิดการลอยของหน้าต่าง
    .toggle-orientation = สลับการวางแนวหน้าต่าง

replace-shortcut-dialog = แทนที่คีย์ลัดหรือไม่
    .desc = { $shortcut } ถูกใช้โดย { $name } หากคุณแทนที่มัน { $name } จะถูกปิด

## Input: Mouse

mouse = เมาส์
    .desc = ความเร็วเมาส์ ความเร่ง และการเลื่อนอย่างเป็นธรรมชาติ
    .speed = ความเร็วเมาส์
    .acceleration = เปิดใช้งานการเร่งความเร็วเมาส์

## Input: Touchpad

click-behavior = พฤติกรรมการคลิก
    .click-finger = การคลิกรองด้วยสองนิ้วและคลิกกลางด้วยสามนิ้ว
    .button-areas = คลิกรองในมุมล่างขวาและคลิกกลางบริเวณล่างกลาง

pinch-to-zoom = บีบเพื่อซูม
    .desc = ใช้สองนิ้วเพื่อซูมเนื้อหา สำหรับแอพลิเคชั่นที่รองรับการซูม

tap-to-click = แตะเพื่อคลิก
    .desc = เปิดการแตะนิ้วเดียวเพื่อการคลิกหลัก แตะสองนิ้วเพื่อการคลิกรอง แตะสามนิ้วเพื่อคลิกกลาง

touchpad = ทัชแพด
    .acceleration = เปิดใช้งานการเร่งความเร็วทัชแพด
    .desc = ความเร็วทัชแพด ตัวเลือกการคลิก และท่าทาง
    .speed = ความเร็วทัชแพด

## Input: Gestures

gestures = ท่าทาง
    .four-finger-down = สี่นิ้วปัดลง
    .four-finger-left = สี่นิ้วปัดไปทางซ้าย
    .four-finger-right = สี่นิ้วปัดไปทางขวา
    .four-finger-up = สี่นิ้วปัดขึ้น
    .three-finger-any = สามนิ้วปัดไปทางใดก็ได้

switch-workspaces = สลับพื้นที่ทำงาน
    .horizontal = เลื่อนสี่นิ้วซ้าย/ขวา
    .vertical = เลื่อนสี่นิ้วขึ้น/ลง

switch-between-windows = สลับระหว่างหน้าต่าง
open-application-library = เปิดไลบรารีแอพลิเคชั่น
open-workspaces-view = เปิดภาพรวมพื้นที่ทำงาน

## Time & Language

time = เวลาและภาษา
    .desc = N/A

time-date = วันที่และเวลา
    .desc = โซนเวลา การตั้งค่านาฬิกาอัตโนมัติ และรูปแบบเวลา
    .auto = ตั้งโดยอัตโนมัติ
    .auto-ntp = วันที่และเวลาจะอัพเดทอัตโนมัติเมื่อโซนเวลาถูกตั้งอยู่

time-zone = โซนเวลา
    .auto = โซนเวลาอัตโนมัติ
    .auto-info = จำเป็นต้องใช้บริการบอกตำแหน่งและการเข้าถึงอินเทอร์เน็ต

time-format = รูปแบบวันที่และเวลา
    .twenty-four = เวลาแบบ 24 ชั่วโมง
    .show-seconds = แสดงวินาที
    .first = วันแรกของสัปดาห์
    .show-date = แสดงวันที่บนแผงด้านบน
    .friday = ศุกร์
    .saturday = เสาร์
    .sunday = อาทิตย์
    .monday = จันทร์

time-region = ภาษาและภูมิภาค
    .desc = รูปแบบวันที่ เวลา และตัวเลขตามภูมิภาคของคุณ

formatting = การจัดรูปแบบ
    .dates = วันที่
    .time = เวลา
    .date-and-time = วันที่และเวลา
    .numbers = ตัวเลข
    .measurement = การวัด
    .paper = กระดาษ

preferred-languages = ภาษาที่ต้องการ
    .desc = ลำดับของภาษากำหนดภาษาที่จะถูกใช้สำหรับการแปลภาษาเดสก์ทอป การเปลี่ยนแปลงจะมีผลในการเข้าสู่ระบบครั้งต่อไป

add-language = Add language
    .context = Add Language
install-additional-languages = Install additional languages
region = Region

## System

system = ระบบและบัญชี

## System: About

about = เกี่ยวกับ
    .desc = ชื่ออุปกรณ์ ข้อมูลฮาร์ดแวร์ และค่าเริ่มต้นระบบปฏิบัติการ

about-device = ชื่ออุปกรณ์
    .desc = ชื่อนี้แสดงต่ออุปกรณ์ภายในเครือข่ายหรืออุปกรณ์บลูธูทอื่น ๆ

about-hardware = ฮาร์ดแวร์
    .model = รุ่นฮาร์ดแวร์
    .memory = หน่วยความจำ
    .processor = โปรเซสเซอร์
    .graphics = กราฟิก
    .disk-capacity = ความจุดิสก์

about-os = ระบบปฏิบัติการ
    .os = ระบบปฏิบัติการ
    .os-architecture = สถาปัตยกรรมระบบปฏิบัติการ
    .desktop-environment = สภาพแวดล้อมเดสก์ท็อป
    .windowing-system = ระบบหน้าต่าง

about-related = การตั้งค่าที่เกี่ยวข้อง
    .support = รับความช่วยเหลือ

## System: Firmware

firmware = เฟิร์มแวร์
    .desc = รายละเอียดเฟิร์มแวร์

## System: Users

users = ผู้ใช้
    .desc = การเข้าสู่ระบบและบัญชีผู้ใช้
    .admin = ผู้ดูแลระบบ
    .standard = มาตรฐาน
    .profile-add = เลือกรูปโปรไฟล์

administrator = ผู้ดูแลระบบ
    .desc = ผู้ดูแลระบบสามารถเปลี่ยนแปลงการตั้งค่าสำหรับผู้ใช้ทั้งหมดได้ เพิ่มและลบผู้ใช้อื่นได้

add-user = เพิ่มผู้ใช้
remove-user = ลบผู้ใช้
full-name = ชื่อเต็ม
username = ชื่อผู้ใช้
password = รหัสผ่าน

## System: Default Applications

default-apps = แอพลิเคชั่นเริ่มต้น
    .desc = ค่าเริ่มต้นแอพลิเคชั่นเว็บเบราเซอร์ ไคลเอนต์อีเมล ตัวจัดการไฟล์ และแอพลิเคชั่นอื่น ๆ
    .web-browser = เว็บเบราเซอร์
    .file-manager = ตัวจัดการไฟล์
    .mail-client = ไคลเอนต์อีเมล
    .music = เพลง
    .video = วีดีโอ
    .photos = รูปภาพ
    .calendar = ปฏิทิน
    .terminal = เทอร์มินัล
    .other-associations = โปรแกรมที่เกี่ยวข้องกับรูปแบบไฟล์อื่น ๆ
