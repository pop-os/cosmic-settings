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
airplane-on = وضع الطائرة مفعل.
cable-unplugged = الكابل مفصول
connect = توصيل
connected = متصل
connecting = جاري الاتصال…
disconnect = قطع الاتصال
forget = انسى
known-networks = الشبكات المعروفة
network-and-wireless = الشبكة واللاسلكي
no-networks = لم يتم العثور على أي شبكات.
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
confirm = تأكيد
enable = تمكين
bluetooth = بلوتوث
    .desc = إدارة أجهزة البلوتوث
    .status = هذا النظام مرئي كـ { $aliases } أثناء فتح إعدادات البلوتوث.
    .connected = متصل
    .connecting = جاري الاتصال
    .disconnecting = جاري قطع الاتصال
    .connect = اتصال
    .disconnect = قطع الاتصال
    .forget = نسيان
    .dbus-error = حدث خطأ أثناء التفاعل مع DBus: { $why }
    .disabled = خدمة البلوتوث معطلة
    .inactive = خدمة البلوتوث غير مفعلة
    .unknown = تعذر تفعيل خدمة البلوتوث. هل BlueZ مثبت؟
bluetooth-paired = الأجهزة المتصلة سابقًا
    .connect = اتصال
    .battery = { $percentage }٪ مُدَّخرة
bluetooth-confirm-pin = تأكيد رقم التعريف الشخصي للبلوتوث
    .description = يرجى التأكد من أن رقم التعريف الشخصي التالي يطابق الرقم المعروض على { $device }
bluetooth-available = الأجهزة القريبة
bluetooth-adapters = محولات البلوتوث

## Accessibility

accessibility = إمكانية الوصول
    .vision = الرؤية
    .on = تشغيل
    .off = إيقاف
    .unavailable = غير متوفر
    .screen-reader = قارئ الشاشة
    .high-contrast = وضع التباين العالي
    .invert-colors = عكس الألوان
    .color-filters = مرشحات الألوان
hearing = السمع
    .mono = تشغيل الصوت الاستريو كصوت أحادي
default = الافتراضي
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
        Super + التمرير بالماوس
    .scroll_controls = تمكين التكبير بالماوس أو لوحة اللمس باستخدام Super + Scroll
    .show_overlay = إظهار تراكب المكبر
    .increment = زيادة التكبير
    .signin = بدء تشغيل المكبر عند تسجيل الدخول
    .applet = تشغيل/إيقاف تشغيل المكبر في التطبيق الصغير على اللوحة
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
add-color = إضافة لون
add-image = إضافة صورة
all-displays = كل الشاشات
colors = الألوان
dialog-add = إضافة
fill = ملء
fit-to-screen = ملائمة للشاشة
open-new-folder = فتح مجلد جديد
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
    .desc = ألوان التمييز والثيمات.
accent-color = لون التمييز
app-background = خلفية النافذة
auto = تلقائي
close = أغلق
color-picker = مُنتقي اللون
copied-to-clipboard = تم النسخ إلى الحافظة
copy-to-clipboard = نسخ إلى الحافظة
dark = داكن
export = تصدير
hex = Hex
import = استيراد
light = فاتح
mode-and-colors = الوضع والألوان
recent-colors = الألوان الحديثة
reset-to-default = إعادة الضبط إلى الإعدادات الافتراضية
rgb = RGB
window-hint-accent = لون تبريز النافذة النشطة
window-hint-accent-toggle = استخدام لون تمييز السمة كتبريز النافذة النشطة
auto-switch = التبديل تلقائيًا بين الوضعين الفاتح والداكن
    .sunrise = يبدِّل إلى الوضع الفاتح عند شروق الشمس
    .sunset = يبدِّل إلى الوضع الداكن عند غروب الشمس
    .next-sunrise = يبدِّل إلى الوضع الفاتح عند شروق الشمس المقبل
    .next-sunset = يبدِّل إلى الوضع الداكن عند غروب الشمس المقبل
container-background = خلفية الحاوية
    .desc-detail = يستخدم لون خلفية الحاوية في شريط التنقل الجانبي، والدرج الجانبي، ومربعات الحوار، وعناصر واجهة المستخدم المماثلة. بشكل افتراضي، يتم اشتقاقه تلقائيًا من خلفية التطبيق أو النافذة.
    .reset = إعادة الضبط إلى الوضع التلقائي
    .desc = يستخدم لون الحاوية الأساسي في شريط التنقل الجانبي، والدرج الجانبي، ومربعات الحوار، وعناصر واجهة المستخدم المماثلة.
control-tint = تحكم في لون المكون
    .desc = يستخدم لخلفيات الأزرار القياسية ومدخلات البحث ومدخلات النص والمكونات المماثلة.
frosted = تأثير الزجاج البلوري على واجهة النظام
    .desc = يطبق ضبابية الخلفية على شريط المهام، وحامل التطبيقات، والتطبيقات المصغرة، ومشغل التطبيقات، ومكتبة التطبيقات.
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
    .active-hint = حجم تبريز النافذة النشطة
    .gaps = الفجوات حول النوافذ المقسمة

### Experimental

experimental-settings = الإعدادات التجريبية
icons-and-toolkit = ثيم الأيقونات وعدد الأدوات المستخدمة
interface-font = خط النظام
monospace-font = خط أحادي المسافة

## Desktop: Notifications

notifications = الإشعارات
    .desc = لا تزعج، وإشعارات شاشة القفل، وإعدادات كل تطبيق على حدة.

## Desktop: Panel

panel = شريط المهام
    .desc = شريط النظام الأساسي للقوائم والتطبيقات المصغرة.
add = إضافة
add-applet = إضافة برنامج مصغر
all = الكل
applets = البرامج المصغرة
center-segment = الجزء الأوسط
end-segment = جزء النهاية
large = كبير
no-applets-found = لم يتم العثور على أي برامج مصغرة...
panel-bottom = الأسفل
panel-left = اليسار
panel-right = اليمين
panel-top = الأعلى
search-applets = بحث عن البرامج المصغرة...
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
panel-style = المظهر
    .anchor-gap = المساحة بين حواف شريط المهام والشاشة
    .dock-anchor-gap = المساحة بين حواف حامل التطبيقات والشاشة
    .extend = تمديد شريط المهام إلى حواف الشاشة
    .dock-extend = تمديد حامل التطبيقات إلى حواف الشاشة
    .appearance = المظهر
    .size = الحجم
    .background-opacity = شفافية الخلفية
panel-applets = الإعدادات
    .dock-desc = إعداد تطبيقات الحامل المصغرة
    .desc = إعداد تطبيقات شريط المهام المصغرة
panel-missing = إعدادات شريط المهام مفقودة
    .desc = ملف إعداد شريط المهام مفقود بسبب استخدام إعداد مخصص أو أنه تالف.
    .fix = إعادة التعيين إلى الإعدادات الافتراضية

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
    .focus-follows-cursor-delay = التركيز يتبع المؤشر بتأخير بالمللي ثانية
    .cursor-follows-focus = المؤشر يتبع التركيز

## Desktop: Workspaces

workspaces = مساحات العمل
    .desc = توجه وسلوك مساحة العمل.
workspaces-behavior = سلوك مساحة العمل
    .dynamic = مساحات عمل ديناميكية
    .dynamic-desc = يزيل مساحات العمل الفارغة تلقائيًا.
    .fixed = عدد ثابت من مساحات العمل
    .fixed-desc = إضافة أو إزالة مساحات العمل في النظرة العامة.
workspaces-multi-behavior = سلوك الشاشات المتعددة
    .span = مساحات العمل تمتد عبر الشاشات
    .separate = الشاشات لها مساحات عمل منفصلة
workspaces-overview-thumbnails = صور مساحة العمل المصغرة
    .show-number = إظهار رقم مساحة العمل
    .show-name = إظهار اسم مساحة العمل
workspaces-orientation = توجيه مساحات العمل
    .vertical = عمودي
    .horizontal = أفقي
hot-corner = الزاوية الساخنة
    .top-left-corner = تمكين الزاوية الساخنة العلوية اليسرى لمساحات العمل

## Displays

-requires-restart = يتطلب إعادة التشغيل
color = اللون
    .depth = عمق اللون
    .profile = ملف تعريف اللون
    .sidebar = ملفات تعريف الألوان
    .temperature = درجة حرارة اللون
display = الشاشات
    .desc = إدارة الشاشات والإضاءة الليلية
    .arrangement = ترتيب الشاشات
    .arrangement-desc = اسحب الشاشات لإعادة ترتيبها.
    .enable = تمكين الشاشة
    .external = { $size } { $output } شاشة خارجية
    .laptop = { $size } شاشة الكمبيوتر المحمول
    .options = خيارات الشاشة
    .refresh-rate = معدل التحديث
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
    .auto = تلقائي (من غروب الشمس إلى شروقها)
    .desc = تقليل الضوء الأزرق بألوان أكثر دفئًا.
orientation = التوجيه
    .standard = قياسي
    .rotate-90 = تدوير 90 درجة
    .rotate-180 = تدوير 180 درجة
    .rotate-270 = تدوير 270 درجة
vrr = معدل التحديث المتغير
    .enabled = ممكّن
    .force = دائمًا
    .auto = تلقائي
    .disabled = معطل
scheduling = الجدولة
    .manual = جدولة يدوية
dialog = حوار
    .title = هل تريد الاحتفاظ بإعدادات العرض هذه؟
    .keep-changes = الاحتفاظ بالتغييرات
    .change-prompt = سيتم التراجع عن تغييرات الإعدادات تلقائيًا في غضون { $time } ثانية.
    .revert-settings = التراجع عن الإعدادات

## Sound

sound = الصوت
    .desc = غير متوفر
sound-output = الإخراج
    .volume = مستوي صوت الإخراج
    .device = جهاز الإخراج
    .level = مستوى الإخراج
    .config = التكوين
    .balance = التوازن
    .left = اليسار
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
profile = ملف التعريف

## Power

power = الطاقة ومُدَّخرتُها
    .desc = إدارة إعدادات الطاقة
battery = المُدَّخرة
    .minute =
        { $value } { $value ->
            [one] minute
           *[other] minutes
        }
    .hour =
        { $value } { $value ->
            [one] hour
           *[other] hours
        }
    .day =
        { $value } { $value ->
            [one] day
           *[other] days
        }
    .less-than-minute = أقل من دقيقة
    .and = و
    .remaining-time =
        { $time } حتى { $action ->
            [full] ممتلئة
           *[other] فارغة
        }
connected-devices = الأجهزة المتصلة
    .unknown = جهاز غير معروف
power-mode = وضع الطاقة
    .battery = عمر مُدَّخرة طويل
    .battery-desc = استهلاك طاقة منخفض وأداء صامت.
    .balanced = متوازن
    .balanced-desc = أداء واستهلاك طاقة معتدل.
    .performance = أداء عالي
    .performance-desc = أداء واستهلاك طاقة مرتفع.
    .no-backend = لم يتم العثور على الخلفية. ثَبِّت system76-power أو power-profiles-daemon.
power-saving = خيارات توفير الطاقة
    .turn-off-screen-after = إيقاف تشغيل الشاشة بعد
    .auto-suspend = التعليق التلقائي
    .auto-suspend-ac = التعليق التلقائي عند التوصيل بالكهرباء
    .auto-suspend-battery = التعليق التلقائي عند استخدام المُدَّخرة

## Input

acceleration-desc = يضبط حساسية التتبع تلقائيًا بناءً على السرعة.
disable-while-typing = تعطيل أثناء الكتابة
input-devices = أجهزة الإدخال
    .desc = أجهزة الإدخال
primary-button = الزر الأساسي
    .desc = يحدد ترتيب الأزرار .
    .left = يسار
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
    .desc = مصادر الإدخال، التبديل، إدخال الأحرف الخاصة، الاختصارات.
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
    .compose = مفتاح التكوين
    .caps = مفتاح Caps Lock
keyboard-typing-assist = الكتابة
    .repeat-rate = معدل التكرار
    .repeat-delay = تأخير التكرار
added = مضاف
type-to-search = اكتب للبحث...
show-extended-input-sources = إظهار مصادر الإدخال الممتدة

## Input: Keyboard: Shortcuts

keyboard-shortcuts = اختصارات لوحة المفاتيح
    .desc = عرض وتخصيص الاختصارات
add-another-keybinding = إضافة إختصار مفتاح آخر
cancel = إلغاء
command = الأمر
custom = مخصص
debug = تصحيح الأخطاء
disabled = غير مفعل
input-source-switch = تبديل مصدر إدخال لغة لوحة المفاتيح
migrate-workspace-prev = ترحيل مساحة العمل إلى الإخراج السابق
migrate-workspace-next = ترحيل مساحة العمل إلى الإخراج التالي
migrate-workspace =
    ترحيل مساحة العمل إلى الإخراج { $direction ->
       *[down] الأسفل
        [left] اليسار
        [right] اليمين
        [up] الأعلى
    }
navigate = تنقل
replace = استبدال
shortcut-name = اسم الاختصار
system-controls = إعدادات النظام
terminate = تدمير
toggle-stacking = تفعيل تكديس النوافذ
type-key-combination = اكتب مجموعة المفاتيح
custom-shortcuts = اختصارات مخصصة
    .add = إضافة اختصار
    .context = إضافة اختصار مخصص
    .none = لا توجد اختصارات مخصصة
modified = تم تعديل { $count }
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
            [left] يسار
            [out] خارج
            [right] يمين
            [up] أعلى
        }
    .output =
        التبديل إلى الإخراج { $direction ->
           *[down] أسفل
            [left] يسار
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
            [left] لليسار
            [right] لليمين
            [up] لأعلى
        }
    .display =
        نقل النافذة إلى شاشة واحدة { $direction ->
           *[down] لأسفل
            [left] لليسار
            [right] لليمين
            [up] لأعلى
        }
    .workspace =
        نقل النافذة إلى مساحة عمل واحدة { $direction ->
           *[below] أسفل
            [left] يسار
            [right] يمين
            [above] أعلى
        }
    .workspace-num = نقل النافذة إلى مساحة العمل { $num }
    .prev-workspace = نقل النافذة إلى مساحة العمل السابقة
    .next-workspace = نقل النافذة إلى مساحة العمل التالية
    .last-workspace = نقل النافذة إلى مساحة العمل الأخيرة
    .next-display = نقل النافذة إلى الشاشة التالية
    .prev-display = نقل النافذة إلى الشاشة السابقة
    .send-to-prev-workspace = نقل النافذة إلى مساحة العمل السابقة
    .send-to-next-workspace = نقل النافذة إلى مساحة العمل التالية
system-shortcut = النظام
    .app-library = فتح مكتبة التطبيقات
    .brightness-down = خفض سطوع الشاشة
    .brightness-up = زيادة سطوع الشاشة
    .display-toggle = تفعيل الشاشة الداخلية
    .home-folder = فتح المجلد الرئيسي
    .keyboard-brightness-down = خفض سطوع لوحة المفاتيح
    .keyboard-brightness-up = زيادة سطوع لوحة المفاتيح
    .launcher = فتح مشغل التطبيقات
    .log-out = تسجيل الخروج
    .lock-screen = قفل الشاشة
    .mute = كتم صوت الإخراج الصوتي
    .mute-mic = كتم صوت الميكروفون
    .play-pause = تشغيل/إيقاف مؤقت
    .play-next = المقطع التالي
    .play-prev = المقطع السابق
    .poweroff = إيقاف التشغيل
    .screenshot = التقاط لقطة شاشة
    .terminal = فتح الطرفية
    .touchpad-toggle = تفعيل لوحة اللمس
    .volume-lower = خفض مستوى صوت الإخراج الصوتي
    .volume-raise = زيادة مستوى صوت الإخراج الصوتي
    .web-browser = فتح متصفح ويب
    .window-switcher = التبديل بين النوافذ المفتوحة
    .window-switcher-previous = التبديل بين النوافذ المفتوحة في الاتجاه المعاكس
    .workspace-overview = فتح نظرة عامة على مساحة العمل
window-tiling = تقسيم النوافذ
    .horizontal = تعيين الاتجاه الأفقي
    .vertical = تعيين الاتجاه الرأسي
    .swap-window = تبديل النوافذ
    .toggle-tiling = تفعيل وضع تقسيم النوافذ
    .toggle-stacking = تفعيل وضع تكديس النوافذ
    .toggle-floating = تفعيل وضع طفو النوافذ
    .toggle-orientation = تبديل الاتجاه
replace-shortcut-dialog = استبدال الاختصار؟
    .desc = { $shortcut } يُستخدم من قبل { $name }. إذا قمت باستبداله، سيتم تعطيل { $name }.
zoom-in = تقريب
zoom-out = تقليص

## Input: Mouse

mouse = الماوس
    .desc = سرعة الماوس، التسارع، التمرير الطبيعي.
    .speed = سرعة الماوس
    .acceleration = تمكين تسارع الماوس

## Input: Touchpad

click-behavior = سلوك النقر
    .click-finger = النقر الثانوي بإصبعين والنقر الأوسط بثلاثة أصابع
    .button-areas = النقر الثانوي في الزاوية اليمنى السفلية والنقر الأوسط في المنتصف السفلي
pinch-to-zoom = الضغط للتكبير
    .desc = استخدم إصبعين لتقريب المحتوى، في التطبيقات التي تدعم ذلك.
tap-to-click = اضغط للنقر
    .desc = يتيح النقر بإصبع واحد للنقر الأساسي، والنقر بإصبعين للنقر الثانوي، والنقر بثلاثة أصابع للنقر الأوسط.
touchpad = لوحة اللمس
    .acceleration = يمكن تسارع لوحة اللمس
    .desc = سرعة لوحة اللمس، وخيارات النقر، والإيماءات.
    .speed = سرعة لوحة اللمس

## Input: Gestures

gestures = الإيماءات
    .four-finger-down = تمرير بأربعة أصابع لأسفل
    .four-finger-left = تمرير بأربعة أصابع لليسار
    .four-finger-right = تمرير بأربعة أصابع لليمين
    .four-finger-up = تمرير بأربعة أصابع لأعلى
    .three-finger-any = تمرير بثلاثة أصابع في أي اتجاه
switch-workspaces = تبديل مساحات العمل
    .horizontal = تمرير بأربعة أصابع إلى اليسار/اليمين
    .vertical = تمرير بأربعة أصابع إلى الأعلى/الأسفل
switch-between-windows = التبديل بين النوافذ
open-application-library = فتح مكتبة التطبيقات
open-workspaces-view = نظرة عامة على مساحات العمل المفتوحة

## Time & Language

time = الوقت واللغة
    .desc = غير متوفر
time-date = التاريخ والوقت
    .desc = المنطقة الزمنية، إعدادات الساعة التلقائية، وبعض تنسيقات الوقت.
    .auto = الضبط التلقائي
    .auto-ntp = سيتم تحديث التاريخ والوقت تلقائيًا عند ضبط المنطقة الزمنية.
time-zone = المنطقة الزمنية
    .auto = المنطقة الزمنية التلقائية
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
    .desc = يحدد ترتيب اللغات اللغة المستخدمة في واجهة المستخدم. تسري التغييرات عند تسجيل الدخول التالي.
add-language = إضافة لغة
    .context = إضافة لغة
install-additional-languages = تثبيت لغات إضافية
region = المنطقة

## Applications

applications = التطبيقات

## Applications: Default Applications

default-apps = التطبيقات الافتراضية
    .desc = متصفح الويب الافتراضي، خدمة البريد الإلكتروني، مدير الملفات، والتطبيقات الأخرى.
    .web-browser = متصفح الويب
    .file-manager = مدير الملفات
    .mail-client = عميل البريد الإلكتروني
    .music = الموسيقى
    .video = الفيديو
    .photos = الصور
    .calendar = التقويم
    .terminal = الطرفية
    .other-associations = الارتباطات الأخرى
    .text-editor = محرر النصوص

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

legacy-applications = توافق تطبيقات X11
    .desc = تغيير حجم تطبيقات نظام النوافذ X11 والاختصارات العامة.
legacy-app-global-shortcuts = اختصارات عامة في تطبيقات X11
    .desc = تسمح الاختصارات العامة بالتعرف على ضغطات المفاتيح وأحداث أزرار الماوس التي يتم تنفيذها في التطبيقات من قبل تطبيقات أخرى لخصائص مثل الضغط للتحدث أو الضغط لكتم الصوت. بشكل افتراضي، يتم تعطيل هذه الميزة في تطبيقات X11 لضمان عدم تمكن التطبيقات الأخرى من مراقبة أحداث لوحة المفاتيح والماوس التي تحتوي على معلومات حساسة.
    .none = لا توجد مفاتيح
    .modifiers = المعدلات (Super، Shift، Control، Alt)
    .combination = جميع المفاتيح أثناء الضغط على المعدلات Super أو Control أو Alt
    .all = جميع المفاتيح
    .mouse = أحداث أزرار الماوس في تطبيقات X11
legacy-app-scaling = تحجيم تطبيقات نظام النوافذ X11
    .scaled-gaming = تحسين الألعاب والتطبيقات التي تعمل بوضع ملء الشاشة
    .gaming-description = قد تظهر تطبيقات X11 أكبر/أصغر قليلاً مقارنة بتطبيقات Wayland.
    .scaled-applications = تحسين التطبيقات
    .applications-description = قد لا تتطابق الألعاب وتطبيقات X11 التي تعمل بوضع ملء الشاشة مع دقة الشاشة.
    .scaled-compatibility = وضع التوافق الأقصى
    .compatibility-description = قد تظهر تطبيقات X11 ضبابية على شاشات HiDPI.
    .preferred-display = الشاشة المفضلة للألعاب وتطبيقات X11 بملء الشاشة
    .no-display = لا شيء

## System

system = النظام والحسابات

## System: About

about = حول
    .desc = اسم الجهاز، معلومات الهاردوير، إعدادات نظام التشغيل الافتراضية.
about-device = اسم الجهاز
    .desc = هذا الاسم يظهر للشبكات الأخرى وأجهزة بلوتوث.
about-hardware = هاردوير
    .model = طراز الجهاز
    .memory = الذاكرة
    .processor = المعالج
    .graphics = كارت الشاشة
    .disk-capacity = مساحة التخزين
about-os = نظام التشغيل
    .os = نظام التشغيل
    .os-architecture = معمارية نظام التشغيل
    .desktop-environment = بيئة سطح المكتب
    .windowing-system = نظام النوافذ
about-related = الإعدادات ذات الصلة
    .support = الحصول على الدعم

## System: Firmware

firmware = البرمجيات الثابتة
    .desc = تفاصيل البرمجيات الثابتة.

## System: Users

users = المستخدمون
    .desc = المصادقة وحسابات المستخدمين.
    .admin = مدير
    .standard = عادي
    .profile-add = اختر صورة الملف الشخصي
administrator = مدير
    .desc = يمكن للمديرين تغيير الإعدادات لجميع المستخدمين، وإضافة مستخدمين آخرين وإزالتهم.
add-user = إضافة مستخدم
change-password = تغيير كلمة السر
remove-user = أزِل المستخدم
full-name = الاسم الكامل
invalid-username = اسم المستخدم غير صالح.
password-mismatch = يجب أن تتطابق كلمة السر والتأكيد.
save = حفظ
never = أبداً
keyboard-numlock-boot = Numlock
    .boot-state = الحالة عند التشغيل
    .last-boot = آخر تشغيل
    .on = تشغيل
    .off = إيقاف
    .set = تعيين حالة تشغيل Numlock
