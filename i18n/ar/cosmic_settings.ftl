app = إعدادات COSMIC
dbus-connection-error = تعذر الاتصال بـDBus
ok = حسنًا
unknown = مجهول
number = { $number }

## Network & Wireless

connections-and-profiles =
    { $variant ->
        [wired] سلكي
        [wifi] واي فاي
        [vpn] VPN
       *[other] غير معروف
    } الاتصالات وملفات تعريف الاتصال.
add-network = أضف شبكة
    .profile = أضف ملف تعريف
add-vpn = إضافة VPN
airplane-on = وضع الطائرة مفعَّل.
cable-unplugged = الكابل مفصول
connect = توصيل
connected = متصل
connecting = يتصل…
disconnect = اقطع الاتصال
forget = انسَ
known-networks = الشبكات المعروفة
network-and-wireless = الشبكة واللاسلكي
no-networks = لم يُعثر على أي شبكات.
no-vpn = لا توجد اتصالات VPN متاحة.
password = كلمة السر
password-confirm = أكِّد كلمة السر
remove = أزِل
settings = الإعدادات
username = اسم المستخدم
visible-networks = الشبكات المرئية
identity = الهوية
auth-dialog = المصادقة مطلوبة
    .vpn-description = أدخل اسم المستخدم وكلمة السر المطلوبين من قبل خدمة VPN.
    .wifi-description = أدخل كلمة السر أو مفتاح التشفير. يمكنك أيضًا الاتصال بالضغط على زر ”WPS“ الموجود على الراوتر.
forget-dialog = هل تريد نسيان شبكة الواي فاي هذه؟
    .description = ستحتاج إلى إدخال كلمة السر مرة أخرى لاستخدام شبكة Wi-Fi هذه في المستقبل.
network-device-state =
    .activated = متصل
    .config = يتصل
    .deactivating = يفصل
    .disconnected = مفصول
    .failed = فشل الاتصال
    .ip-check = يتحقق من الاتصال
    .ip-config = طلب معلومات IP والتوجيه
    .need-auth = يحتاج إلى مصادقة
    .prepare = التحضير للاتصال
    .secondaries = انتظار الاتصال الثانوي
    .unavailable = غير متاح
    .unknown = حالة غير معروفة
    .unmanaged = غير مُدار
    .unplugged = كابل غير موصول
remove-connection-dialog = إزالة ملف تعريف الاتصال؟
    .vpn-description = ستحتاج إلى إدخال كلمة السر مرة أخرى لاستخدام هذه الشبكة في المستقبل.
    .wired-description = ستحتاج إلى إعادة إنشاء ملف التعريف هذا لاستخدامه في المستقبل.
vpn = VPN
    .connections = اتصالات VPN
    .error = فشل إضافة تكوين VPN
    .remove = إزالة ملف تعريف الاتصال
    .select-file = حدد ملف تكوين VPN
vpn-error = خطأ في VPN
    .config = فشل إضافة تكوين VPN
    .connect = فشل الاتصال بـ VPN
    .connection-editor = فشل محرر الاتصال
    .connection-settings = فشل الحصول على إعدادات الاتصالات النشطة
    .updating-state = فشل تحديث حالة مدير الشبكة
    .wireguard-config-path = مسار ملف غير صالح لتكوين WireGuard
    .wireguard-config-path-desc = يجب أن يكون الملف المختار موجودًا على نظام ملفات محلي.
    .wireguard-device = فشل إنشاء جهاز WireGuard
    .with-password =
        فشل تعيين VPN { $field ->
           *[username] اسم المستخدم
            [password] كلمة السر
            [password-flags] علامات كلمة السر
        } باستخدام nmcli
wired = سلكي
    .adapter = محول سلكي { $id }
    .connections = اتصالات سلكية
    .devices = أجهزة سلكية
    .remove = إزالة ملف تعريف الاتصال
wifi = واي فاي
    .adapter = محول واي فاي { $id }
    .forget = نسيان هذه الشبكة
wireguard-dialog = أضف جهاز WireGuard
    .description = اختر اسم جهاز لتكوين WireGuard.

## Networking: Online Accounts

online-accounts = الحسابات عبر الإنترنت
    .desc = إضافة حسابات، IMAP و SMTP، تسجيلات دخول المؤسسة

# Bluetooth

activate = تفعيل
confirm = أكِّد
enable = فعِّل
bluetooth = بلوتوث
    .desc = أدر أجهزة بلوتوث
    .status = سيظهر النظام باسم { $aliases } حين تُفتح إعدادات بلوتوث.
    .connected = متصل
    .connecting = يتصل
    .disconnecting = يقطع الاتصال
    .connect = اتصل
    .disconnect = اقطع الاتصال
    .forget = انسَ
    .dbus-error = حدث خطأ أثناء التعامل مع DBus: { $why }
    .disabled = خدمة البلوتوث معطّلة
    .inactive = خدمة البلوتوث غير نشطة
    .unknown = تعذر تنشيط خدمة البلوتوث. هل BlueZ مُنصَّب؟
bluetooth-paired = الأجهزة المتصلة مسبقًا
    .connect = اتصل
    .battery = المُدَّخرة { $percentage }٪
bluetooth-confirm-pin = تأكيد رقم التعريف الشخصي للبلوتوث
    .description = يرجى التأكد من أن رقم التعريف الشخصي التالي يطابق الرقم المعروض على { $device }
bluetooth-available = الأجهزة القريبة
bluetooth-adapters = محولات البلوتوث

## Accessibility

accessibility = الإتاحة
    .vision = الرؤية
    .on = شغِّل
    .off = أوقف
    .unavailable = غير متوفر
    .screen-reader = قارئ الشاشة
    .high-contrast = وضع التباين العالٍ
    .invert-colors = عكس الألوان
    .color-filters = تصفيات الألوان
hearing = السمع
    .mono = شغَّل الصوت الاستريو كأحادي
default = المبدئي
magnifier = المكبر
    .controls =
        أو استخدم هذه الاختصارات: { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } للتكبير،
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } للتصغير،
        }
        Super + التمرير بالفأرة
    .scroll_controls = فعِّل التكبير بالفأرة أو لوحة اللمس باستخدام Super + Scroll
    .show_overlay = أظهر تراكب المكبر
    .increment = زِد التكبير
    .signin = بدء تشغيل المكبر عند تسجيل الدخول
    .applet = تشغيل/إيقاف تشغيل المكبر في بريمج على اللوحة
    .movement = تحريك العرض المكبر
    .continuous = بشكل مستمر باستخدام المؤشر
    .onedge = عندما يصل المؤشر إلى الحافة
    .centered = للحفاظ على المؤشر في المنتصف
color-filter = نوع تصفية الألوان
    .unknown = تصفية غير معروفة نشطة
    .greyscale = تدرّج رمادي
    .deuteranopia = أخضر/أحمر (ضعف في الأخضر، الديوترانوبيا)
    .protanopia = أحمر/أخضر (ضعف في الأحمر، البروتانوبيا)
    .tritanopia = أزرق/أصفر (ضعف في الأزرق، التريتانوبيا)

## Desktop

desktop = سطح المكتب

## Desktop: Wallpaper

wallpaper = خلفية
    .change = غيِّر الصورة كل
    .desc = صور الخلفية والألوان وخيارات عرض الشرائح.
    .fit = ملاءمة الخلفية
    .folder-dialog = اختر مجلد الخلفية
    .image-dialog = اختر صورة الخلفية
    .plural = خلفيات
    .same = نفس الخلفية على جميع الشاشات
    .slide = عرض الشرائح
add-color = أضِف لون
add-image = أضِف صورة
all-displays = كل الشاشات
colors = الألوان
dialog-add = أضِف
fill = ملء
fit-to-screen = ملائمة للشاشة
open-new-folder = افتح مجلد جديد
recent-folders = المجلدات الحديثة
x-minutes =
    { $number } { $number ->
        [one] دقيقة
       *[other] دقائق
    }
x-hours =
    { $number } { $number ->
        [one] ساعة
       *[other] ساعات
    }

## Desktop: Appearance

appearance = المظهر
    .desc = ألوان التمييز والنسق
accent-color = لون التمييز
app-background = خلفية النافذة
auto = آلي
close = أغلق
color-picker = مُنتقي اللون
copied-to-clipboard = نُسِخ إلى الحافظة
copy-to-clipboard = انسخ إلى الحافظة
dark = داكن
export = تصدير
hex = Hex
import = استورد
light = فاتح
mode-and-colors = الوضع والألوان
recent-colors = الألوان الحديثة
reset-to-default = استعد المبدئيات
rgb = RGB
window-hint-accent = لون تبريز النافذة النشطة
window-hint-accent-toggle = استخدم لون تمييز النسق كتبريز النافذة النشطة
auto-switch = التبديل الآلي بين الوضعين الفاتح والداكن
    .sunrise = يبدِّل إلى الوضع الفاتح عند شروق الشمس
    .sunset = يبدِّل إلى الوضع الداكن عند غروب الشمس
    .next-sunrise = يبدِّل إلى الوضع الفاتح عند شروق الشمس المقبل
    .next-sunset = يبدِّل إلى الوضع الداكن عند غروب الشمس المقبل
container-background = خلفية الحاوية
    .desc-detail = يستخدم لون خلفية الحاوية في شريط التنقل الجانبي والدرج الجانبي ومربعات الحوار وعناصر واجهة المستخدم المماثلة. مبدئيًا، يُشتق خلفية الحاوية آليًا من خلفية النافذة.
    .reset = صفِّر إلى الوضع الآلي
    .desc = يُستخدم في شريط التنقل الجانبي والدرج الجانبي ومربعات الحوار وعناصر واجهة المستخدم المماثلة
control-tint = تحكم في لون المكون
    .desc = يستخدم لخلفيات الأزرار القياسية ومدخلات البحث ومدخلات النص والمكونات المماثلة.
frosted = تأثير الزجاج البلوري على واجهة النظام
    .desc = يطبِّق تمويه الخلفية للوحة والمرسى والبريمجات والمطلق ومكتبة التطبيقات
enable-export = قم بتطبيق هذا المظهر على تطبيقات GNOME.
    .desc = لا تدعم جميع مجموعات الأدوات التبديل التلقائي. قد تحتاج التطبيقات غير التابعة لـ COSMIC إلى إعادة التشغيل بعد تغيير المظهر.
icon-theme = ثيم الأيقونات
    .desc = يطبق مجموعة مختلفة من الأيقونات على التطبيقات.
text-tint = لون نص الواجهة
    .desc = اللون المستخدم لاشتقاق ألوان نص الواجهة التي تتمتع بتباين كافٍ على الأسطح المختلفة.
style = المظهر
    .round = دائري
    .slightly-round = دائري قليلاً
    .square = مربع
interface-density = كثافة الواجهة
    .comfortable = مريحة
    .compact = صغير ة
    .spacious = واسعة
window-management-appearance = إدارة النوافذ
    .active-hint = حجم تلميح النافذة النشطة
    .gaps = الفراغات حول النوافذ المبلَّطة

### Experimental

experimental-settings = الإعدادات التجريبية
icons-and-toolkit = نسق الأيقونات وعُدّة الأدوات المستخدمة
interface-font = خط النظام
monospace-font = خط أحادي المسافة

## Desktop: Notifications

notifications = الإشعارات
    .desc = لا تزعج، وإشعارات شاشة القفل، وإعدادات كل تطبيق على حدة.

## Desktop: Panel

panel = اللوحة
    .desc = شريط النظام الأساسي للقوائم والبريمجات
add = أضِف
add-applet = أضِف بريمج
all = الكل
applets = البريمجات
center-segment = الجزء الأوسط
end-segment = جزء النهاية
large = كبير
no-applets-found = لم يُعثر على بريمجات…
panel-bottom = الأسفل
panel-left = الشمال
panel-right = اليمين
panel-top = الأعلى
search-applets = ابحث البريمجات…
small = صغير
start-segment = جزء البداية
panel-appearance = المظهر
    .match = مطابق لسطح المكتب
    .light = فاتح
    .dark = داكن
panel-behavior-and-position = السلوك والأماكن
    .autohide = إخفاء شريط المهام تلقائيًا
    .dock-autohide = إخفاء حامل التطبيقات تلقائيًا
    .position = المكان على الشاشة
    .display = العرض على الشاشة
panel-style = الأسلوب
    .anchor-gap = فراغ بين الشريط وحواف الشاشة
    .dock-anchor-gap = فراغ بين المرسى وحواف الشاشة
    .extend = تمديد اللوحة إلى حواف الشاشة
    .dock-extend = تمديد المرسى إلى حواف الشاشة
    .appearance = المظهر
    .size = الحجم
    .background-opacity = عتمة الخلفية
panel-applets = الضبط
    .dock-desc = اضبط بريمجات المرسى
    .desc = اضبط بريمجات اللوحة
panel-missing = ضبط اللوحة مفقود
    .desc = فُقِد ضبط اللوحة لاستخدام ضبط مخصَّص أو لأنه معطوب.
    .fix = صفِّر إلى المبدئي

## Desktop: Dock

dock = حامل التطبيقات
    .desc = حامل اختياري للتطبيقات والبرامج المصغرة.

## Desktop: Window management

window-management = إدارة النوافذ
    .desc = إجراءات مفتاح سوبر، وخيارات التحكم في النوافذ، وخيارات إضافية لتقسيم النوافذ.
super-key = إجراءات مفتاح سوبر
    .launcher = فتح مشغل التطبيقات
    .workspaces = فتح مساحات العمل
    .applications = فتح التطبيقات
    .disable = تعطيل
edge-gravity = تنجذب النوافذ التي تطفو إلى الحواف القريبة
window-controls = تحكمات النوافذ
    .maximize = أظهر زر التكبير
    .minimize = أظهر زر التصغير
    .active-window-hint = أظهر تلميح النافذة النشطة
focus-navigation = تنقُل التركيز
    .focus-follows-cursor = التركيز يتبع المؤشر
    .focus-follows-cursor-delay = التركيز يتبع المؤشر بتأخير بالملي ثانية
    .cursor-follows-focus = المؤشر يتبع التركيز

## Desktop: Workspaces

workspaces = مساحات العمل
    .desc = توجه وسلوك مساحة العمل.
workspaces-behavior = سلوك مساحة العمل
    .dynamic = مساحات عمل حركيَّة
    .dynamic-desc = يُزيل مساحات العمل الفارغة آليًا.
    .fixed = عدد محدّد من مساحات العمل
    .fixed-desc = أضِف أو أزِل مساحات العمل في نظرة العامة.
workspaces-multi-behavior = سلوك الشاشات المتعدّدة
    .span = مساحات العمل تمتد عبر الشاشات
    .separate = الشاشات لها مساحات عمل منفصلة
workspaces-overview-thumbnails = مُصغَّرات منظور مساحة العمل العام
    .show-number = أظهر رقم مساحة العمل
    .show-name = أظهر اسم مساحة العمل
workspaces-orientation = توجيه مساحات العمل
    .vertical = عمودي
    .horizontal = أفقي
hot-corner = الزاوية الساخنة
    .top-left-corner = فعِّل الزاوية الساخنة أعلى الشمال لمساحات العمل

## Displays

-requires-restart = يتطلب إعادة التشغيل
color = اللون
    .depth = عمق اللون
    .profile = ملف تعريف اللون
    .sidebar = ملفات تعريف الألوان
    .temperature = درجة حرارة اللون
display = الشاشات
    .desc = أدر الشاشات والإضاءة الليلية
    .arrangement = ترتيب الشاشات
    .arrangement-desc = اسحب الشاشات لإعادة ترتيبها.
    .enable = فعِّل الشاشة
    .external = { $size } { $output } شاشة خارجية
    .laptop = { $size } شاشة الحاسوب المحمول
    .options = خيارات الشاشة
    .refresh-rate = معدّل التحديث
    .resolution = الدقة
    .scale = المقياس
    .additional-scale-options = خيارات المقياس الإضافية
mirroring = النسخ المتطابق
    .id = النسخ المتطابق { $id }
    .dont = عدم النسخ المتطابق
    .mirror = النسخ المتطابق { $display }
    .project =
        مشروع إلى { $display ->
            [all] جميع الشاشات
           *[other] { $display }
        }
    .project-count =
        مشروع إلى { $count } آخر { $count ->
            [1] شاشة
           *[other] شاشات
        }
night-light = إضاءة ليلية
    .auto = آلي (من غروب الشمس إلى شروقها)
    .desc = تقليل الضوء الأزرق بألوان أكثر دفئًا
orientation = التوجيه
    .standard = قياسي
    .rotate-90 = تدوير 90 درجة
    .rotate-180 = تدوير 180 درجة
    .rotate-270 = تدوير 270 درجة
vrr = معدّل التحديث المتغير
    .enabled = مُفعَّل
    .force = دائمًا
    .auto = آلي
    .disabled = معطّل
scheduling = الجدولة
    .manual = جدولة يدوية
dialog = حوار
    .title = أتريد الاحتفاظ بإعدادات العرض هذه؟
    .keep-changes = احتفظ بالتغييرات
    .change-prompt = سيتم التراجع عن تغييرات الإعدادات آليًا في غضون { $time } ثانية.
    .revert-settings = تراجع عن الإعدادات

## Sound

sound = الصوت
    .desc = غير متوفر
sound-output = الإخراج
    .volume = مستوي صوت الإخراج
    .device = جهاز الإخراج
    .level = مستوى الإخراج
    .config = التضبيط
    .balance = التوازن
    .left = الشمال
    .right = اليمين
sound-input = الإدخال
    .volume = مستوي صوت الإدخال
    .device = جهاز الإدخال
    .level = مستوى الإدخال
amplification = التضخيم
    .desc = يسمح برفع مستوى الصوت إلى 150٪.
sound-alerts = التنبيهات
    .volume = مستوي صوت التنبيهات
    .sound = صوت التنبيهات
sound-applications = التطبيقات
    .desc = مستوى صوت التطبيقات والإعدادات

## Power

power = الطاقة والمُدَّخرة
    .desc = أدر إعدادات الطاقة
battery = المُدَّخرة
    .minute =
        { $value } { $value ->
            [one] دقيقة
           *[other] دقائق
        }
    .hour =
        { $value } { $value ->
            [one] ساعة
           *[other] ساعة
        }
    .day =
        { $value } { $value ->
            [one] يوم
           *[other] يوم
        }
    .less-than-minute = أقل من دقيقة
    .and = و
    .remaining-time =
        { $time } حتى { $action ->
            [full] ممتلئة
           *[other] فارغة
        }
connected-devices = الأجهزة المتَّصلة
    .unknown = جهاز مجهول
power-mode = وضع الطاقة
    .battery = عمر مُدَّخرة أطول
    .battery-desc = استهلاك طاقة منخفض وأداء صامت
    .balanced = متوازن
    .balanced-desc = أداء هادئ واستهلاك طاقة معتدل
    .performance = أداء عالٍ
    .performance-desc = قمة استهلاك الطاقة والأداء
    .no-backend = لم يُعثر على المنتهى الخلفي. نصِّب system76-power أو power-profiles-daemon.
power-saving = خيارات توفير الطاقة
    .turn-off-screen-after = أوقف تشغيل الشاشة بعد
    .auto-suspend = علِّق آليًا
    .auto-suspend-ac = تعليق آلي عند التوصيل بالكهرباء
    .auto-suspend-battery = تعليق آلي عند استخدام المُدَّخرة

## Input

acceleration-desc = يضبط حساسية التتبع آليًا بناءً على السرعة
disable-while-typing = عطِّل أثناء الكتابة
input-devices = أجهزة الإدخال
    .desc = أجهزة الإدخال
primary-button = الزر الأساسي
    .desc = يعيِّن ترتيب الأزرار الملموسة.
    .left = شمال
    .right = يمين
scrolling = التمرير
    .two-finger = التمرير بإصبعين
    .edge = التمرير على طول الحافة بإصبع واحد
    .speed = سرعة التمرير
    .natural = التمرير الطبيعي
    .natural-desc = تمرير المحتوى، بدلاً من العرض

## Input: Keyboard

slow = بطيئ
fast = سريع
short = قصير
long = طويل
keyboard = لوحة المفاتيح
    .desc = مصادر الإدخال والتبديل وإدخال الأحرف الخاصة والاختصارات
keyboard-sources = مصادر الإدخال
    .desc = يمكن تبديل مصادر الإدخال باستعمال تركيبة مفتاحي Super+Space. يمكن أن يُخصّص هذا من خلال إعدادات اختصارات لوحة المفاتيح.
    .move-up = حرِك لأعلى
    .move-down = حرِك لأسفل
    .settings = الإعدادات
    .view-layout = اعرض تخطيط لوحة المفاتيح
    .remove = أزِل
    .add = أضِف مصدر إدخال
keyboard-special-char = إدخال أحرف خاصة
    .alternate = مفتاح الأحرف البديلة
    .compose = مفتاح التركيب
    .compose-desc = مفتاح التركيب يسمح بإدخال مجموعة واسعة من الأحرف. لاستخدامه، اضغط على مفتاح التركيب ثم اكتب تسلسلًا من الأحرف. على سبيل المثال، الضغط على مفتاح التركيب متبوعًا بـ C و o سيدخل الرمز ©، بينما الضغط على مفتاح التركيب متبوعًا بـ a و ‘ سيدخل الحرف á.
    .caps = مفتاح Caps Lock
keyboard-typing-assist = الكتابة
    .repeat-rate = معدل التكرار
    .repeat-delay = تأخير التكرار
added = مُضاف
type-to-search = اكتب للبحث...
show-extended-input-sources = أظهِر مصادر الإدخال الممتدة

## Input: Keyboard: Shortcuts

keyboard-shortcuts = اختصارات لوحة المفاتيح
    .desc = اعرض وخصّص الاختصارات
add-another-keybinding = أضِف اختصارًا آخر
cancel = ألغِ
command = الأمر
custom = مخصّص
debug = نقِّح
disabled = معطَّل
input-source-switch = بدِّل مصدر إدخال لغة لوحة المفاتيح
migrate-workspace-prev = رحِّل مساحة العمل إلى الإخراج السابق
migrate-workspace-next = رحِّل مساحة العمل إلى الإخراج التالي
migrate-workspace =
    رحِّل مساحة العمل إلى الإخراج { $direction ->
       *[down] الأسفل
        [left] الشمال
        [right] اليمين
        [up] الأعلى
    }
navigate = تنقل
replace = استبدل
shortcut-name = اسم الاختصار
system-controls = إعدادات النظام
terminate = أنهِ
toggle-stacking = بدِّل تكديس النوافذ
type-key-combination = اكتب مجموعة المفاتيح
custom-shortcuts = الاختصارات المخصَّصة
    .add = أضِف اختصارًا
    .context = أضِف اختصارًا مخصصًا
    .none = لا اختصارات مخصَّصة
modified = { $count } معدَّل
nav-shortcuts = التنقل
    .prev-output = التركيز على الإخراج السابق
    .next-output = التركيز على الإخراج التالي
    .last-workspace = التركيز على مساحة العمل الأخيرة
    .prev-workspace = التركيز على مساحة العمل السابقة
    .next-workspace = التركيز على مساحة العمل التالية
    .focus =
        التركيز على النافذة { $direction ->
           *[down] أسفل
            [in] داخل
            [left] شمال
            [out] خارج
            [right] يمين
            [up] أعلى
        }
    .output =
        التبديل إلى الإخراج { $direction ->
           *[down] أسفل
            [left] شمال
            [right] يمين
            [up] أعلى
        }
    .workspace = التبديل إلى مساحة العمل { $num }
manage-windows = إدارة النوافذ
    .close = إغلاق النافذة
    .maximize = تكبير النافذة
    .fullscreen = نافذة بملء الشاشة
    .minimize = تصغير النافذة
    .resize-inwards = تغيير حجم النافذة للداخل
    .resize-outwards = تغيير حجم النافذة للخارج
    .toggle-sticky = تفعيل النافذة الثابتة
move-windows = نقل النوافذ
    .direction =
        نقل النافذة { $direction ->
           *[down] لأسفل
            [left] للشمال
            [right] لليمين
            [up] لأعلى
        }
    .display =
        نقل النافذة إلى شاشة واحدة { $direction ->
           *[down] لأسفل
            [left] للشمال
            [right] لليمين
            [up] لأعلى
        }
    .workspace =
        نقل النافذة إلى مساحة عمل واحدة { $direction ->
           *[below] لأسفل
            [left] للشمال
            [right] لليمين
            [above] لأعلى
        }
    .workspace-num = انقل النافذة إلى مساحة العمل { $num }
    .prev-workspace = انقل النافذة إلى مساحة العمل السابقة
    .next-workspace = انقل النافذة إلى مساحة العمل التالية
    .last-workspace = انقل النافذة إلى مساحة العمل الأخيرة
    .next-display = انقل النافذة إلى الشاشة التالية
    .prev-display = انقل النافذة إلى الشاشة السابقة
    .send-to-prev-workspace = انقل النافذة إلى مساحة العمل السابقة
    .send-to-next-workspace = انقل النافذة إلى مساحة العمل التالية
system-shortcut = النظام
    .app-library = فتح مكتبة التطبيقات
    .brightness-down = تقليل سطوع الشاشة
    .brightness-up = زيادة سطوع الشاشة
    .display-toggle = بدّل الشاشة الداخلية
    .home-folder = فتح مجلد المنزل
    .keyboard-brightness-down = تقليل سطوع لوحة المفاتيح
    .keyboard-brightness-up = زيادة سطوع لوح المفاتيح
    .launcher = فتح المطلق
    .log-out = الخروج
    .lock-screen = قفل الشاشة
    .mute = كتم مخرج الصوت
    .mute-mic = كتم مدخل الميكرفون
    .play-pause = التشغيل/الإيقاف
    .play-next = المقطوعة التالية
    .play-prev = المقطوعة السابقة
    .poweroff = أطفئ
    .screenshot = التقاط لقطة شاشة
    .suspend = علِّق
    .terminal = فتح طرفيَّة
    .touchpad-toggle = بدّل لوحة اللمس
    .volume-lower = تقليل مستوى صوت المخرج
    .volume-raise = رفع مستوى صوت المخرج
    .web-browser = فتح متصفح وِب
    .window-switcher = التبديل بين النوافذ
    .window-switcher-previous = تبديل بين النوافذ المفتوحة معكوسًا
    .workspace-overview = فتح منظور مساحة العمل العام
window-tiling = تبليط النوافذ
    .horizontal = تعيين الاتجاه الأفقي
    .vertical = تعيين الاتجاه العمودي
    .swap-window = تبديل النافذة
    .toggle-tiling = تبديل تبليط النافذة
    .toggle-stacking = تبديل تكديس النافذة
    .toggle-floating = تبديل تعويم النافذة
    .toggle-orientation = تبديل الاتجاه
replace-shortcut-dialog = استبدل الاختصار؟
    .desc = { $shortcut } مستخدم من قِبل { $name }. إذا استُبدِل، سيعطَّل { $name }.
zoom-in = قرِّب
zoom-out = بعِّد

## Input: Mouse

mouse = الفأرة
    .desc = سرعة الفأرة والتسارع والتمرير الطبيعي.
    .speed = سرعة الفأرة
    .acceleration = فعِّل تسارع الفأرة

## Input: Touchpad

click-behavior = سلوك النقر
    .click-finger = النقر الثانوي بإصبعين والنقر الأوسط بثلاثة أصابع
    .button-areas = النقر الثانوي في الزاوية اليمنى السفلية والنقر الأوسط في المنتصف السفلي
pinch-to-zoom = الضغط للتكبير
    .desc = استخدم إصبعين لتقريب المحتوى، في التطبيقات التي تدعم ذلك.
tap-to-click = اضغط للنقر
    .desc = يتيح الضغط بإصبع واحد للنقر الأساسي والضغط بإصبعين للنقر الثانوي والضغط بثلاثة أصابع للنقر الأوسط
touchpad = لوحة اللمس
    .acceleration = فعِّل تسارع لوحة اللمس
    .desc = سرعة لوحة اللمس وخيارات النقر والإيماءات.
    .speed = سرعة لوحة اللمس

## Input: Gestures

gestures = الإيماءات
    .four-finger-down = تمرير بأربعة أصابع لأسفل
    .four-finger-left = تمرير بأربعة أصابع للشمال
    .four-finger-right = تمرير بأربعة أصابع لليمين
    .four-finger-up = تمرير بأربعة أصابع لأعلى
    .three-finger-any = تمرير بثلاثة أصابع في أي اتجاه
switch-workspaces = تبديل مساحات العمل
    .horizontal = تمرير بأربعة أصابع إلى الشمال/اليمين
    .vertical = تمرير بأربعة أصابع إلى الأعلى/الأسفل
switch-between-windows = التبديل بين النوافذ
open-application-library = فتح مكتبة التطبيقات
open-workspaces-view = نظرة عامة على مساحات العمل المفتوحة

## Time & Language

time = الوقت واللغة
    .desc = غير متوفر
time-date = التاريخ والوقت
    .desc = المنطقة الزمنية وإعدادات الساعة الآلية وبعض تنسيقات الوقت.
    .auto = الضبط الآلي
    .auto-ntp = سيحدَّث التاريخ والوقت الآلي عند ضبط المنطقة الزمنية
time-zone = المنطقة الزمنية
    .auto = المنطقة الزمنية الآلية
    .auto-info = يتطلب خدمات الموقع والوصول إلى الإنترنت
time-format = تنسيق التاريخ والوقت
    .twenty-four = توقيت 24 ساعة
    .show-seconds = إظهار الثواني
    .first = أول يوم في الأسبوع
    .show-date = إظهار التاريخ في تطبيق الوقت
    .friday = الجمعة
    .saturday = السبت
    .sunday = الأحد
    .monday = الاثنين
time-region = المنطقة واللغة
    .desc = تنسيق التواريخ والأوقات والأرقام بناءً على منطقتك.
formatting = التنسيق
    .dates = التواريخ
    .time = الوقت
    .date-and-time = التاريخ والوقت
    .numbers = الأرقام
    .measurement = القياس
    .paper = الورق
preferred-languages = اللغات المفضلة
    .desc = يحدد ترتيب اللغات اللغة المستخدمة لواجهة المستخدم. تسري التغييرات عند الولوج التالي.
add-language = إضافة لغة
    .context = أضف لغة
install-additional-languages = نصِّب لغات إضافية
region = المنطقة

## Applications

applications = التطبيقات

## Applications: Default Applications

default-apps = التطبيقات المبدئية
    .desc = متصفح الويب المبدئي وعميل البريد ومتصفح الملفات والتطبيقات الأخرى.
    .web-browser = متصفح الوِب
    .file-manager = مدير الملفات
    .mail-client = عميل البريد
    .music = صوتيات
    .video = فيديو
    .photos = صور
    .calendar = تقويم
    .terminal = طرفية
    .other-associations = جمعيات أخرى
    .text-editor = محرِّر نصوص

## Applications: Startup Applications

startup-apps = تطبيقات بدء التشغيل
    .desc = تكوين التطبيقات التي يتم تشغيلها عند تسجيل الدخول.
    .add = إضافة تطبيق
    .user = التطبيقات التي يتم تشغيلها عند تسجيل الدخول
    .none = لم تتم إضافة أي تطبيقات بدء تشغيل
    .remove-dialog-title = إزالة { $name }؟
    .remove-dialog-description = هل تريد حذف تطبيق بدء التشغيل هذا؟
    .search-for-application = البحث عن تطبيق

## Applications: Legacy Applications

legacy-applications = توافق تطبيقات اكس11
    .desc = تغيير مقياس تطبيقات نظام النوافذ اكس11 والاختصارات العامة
legacy-app-global-shortcuts = اختصارات عامة في تطبيقات اكس11
    .desc = تسمح الاختصارات العامة بالتعرّف على ضغطات المفاتيح وأحداث زر الفأرة المُجراة في تطبيقٍ ما لتطبيقاتٍ أخرى — لميزات مثل «اضغط لتتحدث» أو «اضغط لكتم الصوت». مبدئيًا، يُعطّل هذا في تطبيقات اكس11 لضمان عدم تمكن التطبيقات الأخرى من مراقبة أحداث لوحة المفاتيح والفأرة التي تحتوي على معلومات حساسة.
    .none = لا مفاتيح
    .modifiers = مفاتيح التعديل (Super وShift وControl وAlt)
    .combination = جميع المفاتيح أثناء الضغط على مفاتيح التعديل Super أو Control أو Alt
    .all = كل المفاتيح
    .mouse = أحداث زر الفأرة في تطبيقات اكس
legacy-app-scaling = مقياس تطبيقات نظام النوافذ اكس11
    .scaled-gaming = تحسين الألعاب والتطبيقات التي تعمل بوضع ملء الشاشة
    .gaming-description = قد تظهر تطبيقات اكس11 أكبر/أصغر قليلاً مقارنة بتطبيقات Wayland.
    .scaled-applications = تحسين التطبيقات
    .applications-description = قد لا تتطابق الألعاب وتطبيقات اكس11 التي تعمل بوضع ملء الشاشة مع دقة الشاشة.
    .scaled-compatibility = وضع التوافق الأقصى
    .compatibility-description = قد تظهر تطبيقات اكس11 ضبابية على شاشات HiDPI.
    .preferred-display = الشاشة المفضلة للألعاب وتطبيقات اكس11 بملء الشاشة
    .no-display = لا شيء

## System

system = النظام والحسابات

## System: About

about = عن
    .desc = اسم الجهاز ومعلومات العتاد ومبدئيات نظام التشغيل
about-device = اسم الجهاز
    .desc = هذا الاسم يظهر للشبكات الأخرى وأجهزة بلوتوث.
about-hardware = العتاد
    .model = طراز العتاد
    .memory = الذاكرة
    .processor = المعالج
    .graphics = الرسوميات
    .disk-capacity = سعة القرص
about-os = نظام التشغيل
    .os = نظام التشغيل
    .os-architecture = معمارية نظام التشغيل
    .kernel = إصدارة النواة
    .desktop-environment = بيئة سطح المكتب
    .windowing-system = نظام النوافذ
about-related = الإعدادات ذات الصلة
    .support = احصل على الدعم

## System: Firmware

firmware = البرمجيات الثابتة
    .desc = تفاصيل البرمجيات الثابتة.

## System: Users

users = المستخدمون
    .desc = الاستيثاق وحسابات المستخدمين.
    .admin = مدير
    .standard = عادي
    .profile-add = اختر صورة ملف تعريف
administrator = مدير
    .desc = يمكن للمديرين تغيير الإعدادات لجميع المستخدمين، وإضافة مستخدمين آخرين وإزالتهم.
add-user = أضِف مستخدم
change-password = غيَّر كلمة السر
remove-user = أزِل المستخدم
full-name = الاسم الكامل
invalid-username = اسم المستخدم غير صالح.
password-mismatch = يجب أن تتطابق كلمة السر والتأكيد.
save = حفظ
never = أبدًا
keyboard-numlock-boot = Numlock
    .boot-state = الحالة عند التشغيل
    .last-boot = آخر تشغيل
    .on = تشغيل
    .off = إيقاف
    .set = تعيين حالة تشغيل Numlock
sound-device-port-unplugged = مفصول
sound-hd-audio = صوت عالي الدقة
sound-usb-audio = صوت USB
sound-device-profiles = ملفات تعريف الجهاز
shadows-floating = النوافذ العائمة
    .clip = مطابقة زوايا النظام وتطبيق الظلال
shadows-tiling = النوافذ المبلطة
    .clip = مطابقة زوايا النظام
    .shadow = طبِّق الظلال
shadow-and-corners = ظل النافذة وزواياها
