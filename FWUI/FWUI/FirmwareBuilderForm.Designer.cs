namespace FWUI
{
    partial class FirmwareBuilderForm
    {
        private System.ComponentModel.IContainer components = null;
        private System.Windows.Forms.SplitContainer splitMain;
        private System.Windows.Forms.SplitContainer splitTop;
        private System.Windows.Forms.GroupBox grpFiles;
        private System.Windows.Forms.ListBox lstInputFiles;
        private System.Windows.Forms.Button btnAddFile;
        private System.Windows.Forms.Button btnRemoveFile;
        private System.Windows.Forms.GroupBox grpHeader;
        private System.Windows.Forms.Label lblVersion;
        private System.Windows.Forms.TextBox txtVersion;
        private System.Windows.Forms.Label lblDeviceId;
        private System.Windows.Forms.TextBox txtDeviceId;
        private System.Windows.Forms.Label lblBuildDate;
        private System.Windows.Forms.DateTimePicker dtpBuildDate;
        private System.Windows.Forms.Label lblFlags;
        private System.Windows.Forms.NumericUpDown nudFlags;
        private System.Windows.Forms.Button btnBuild;
        private System.Windows.Forms.TextBox txtLog;
        private System.Windows.Forms.OpenFileDialog openFileDialog;

        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        private void InitializeComponent()
        {
            this.components = new System.ComponentModel.Container();
            this.splitMain = new System.Windows.Forms.SplitContainer();
            this.splitTop = new System.Windows.Forms.SplitContainer();
            this.grpFiles = new System.Windows.Forms.GroupBox();
            this.lstInputFiles = new System.Windows.Forms.ListBox();
            this.btnAddFile = new System.Windows.Forms.Button();
            this.btnRemoveFile = new System.Windows.Forms.Button();
            this.grpHeader = new System.Windows.Forms.GroupBox();
            this.lblVersion = new System.Windows.Forms.Label();
            this.txtVersion = new System.Windows.Forms.TextBox();
            this.lblDeviceId = new System.Windows.Forms.Label();
            this.txtDeviceId = new System.Windows.Forms.TextBox();
            this.lblBuildDate = new System.Windows.Forms.Label();
            this.dtpBuildDate = new System.Windows.Forms.DateTimePicker();
            this.lblFlags = new System.Windows.Forms.Label();
            this.nudFlags = new System.Windows.Forms.NumericUpDown();
            this.btnBuild = new System.Windows.Forms.Button();
            this.txtLog = new System.Windows.Forms.TextBox();
            this.openFileDialog = new System.Windows.Forms.OpenFileDialog();
            ((System.ComponentModel.ISupportInitialize)(this.splitMain)).BeginInit();
            this.splitMain.Panel1.SuspendLayout();
            this.splitMain.Panel2.SuspendLayout();
            this.splitMain.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.splitTop)).BeginInit();
            this.splitTop.Panel1.SuspendLayout();
            this.splitTop.Panel2.SuspendLayout();
            this.splitTop.SuspendLayout();
            this.grpFiles.SuspendLayout();
            this.grpHeader.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.nudFlags)).BeginInit();
            this.SuspendLayout();
            // 
            // splitMain
            // 
            this.splitMain.Dock = System.Windows.Forms.DockStyle.Fill;
            this.splitMain.FixedPanel = System.Windows.Forms.FixedPanel.Panel2;
            this.splitMain.Location = new System.Drawing.Point(0, 0);
            this.splitMain.Name = "splitMain";
            this.splitMain.Orientation = System.Windows.Forms.Orientation.Horizontal;
            // 
            // splitMain.Panel1
            // 
            this.splitMain.Panel1.Controls.Add(this.splitTop);
            // 
            // splitMain.Panel2
            // 
            this.splitMain.Panel2.Controls.Add(this.txtLog);
            this.splitMain.Size = new System.Drawing.Size(980, 620);
            this.splitMain.SplitterDistance = 430;
            this.splitMain.TabIndex = 0;
            // 
            // splitTop
            // 
            this.splitTop.Dock = System.Windows.Forms.DockStyle.Fill;
            this.splitTop.Location = new System.Drawing.Point(0, 0);
            this.splitTop.Name = "splitTop";
            // 
            // splitTop.Panel1
            // 
            this.splitTop.Panel1.Controls.Add(this.grpFiles);
            // 
            // splitTop.Panel2
            // 
            this.splitTop.Panel2.Controls.Add(this.grpHeader);
            this.splitTop.Size = new System.Drawing.Size(980, 430);
            this.splitTop.SplitterDistance = 470;
            this.splitTop.TabIndex = 0;
            // 
            // grpFiles
            // 
            this.grpFiles.Controls.Add(this.lstInputFiles);
            this.grpFiles.Controls.Add(this.btnAddFile);
            this.grpFiles.Controls.Add(this.btnRemoveFile);
            this.grpFiles.Dock = System.Windows.Forms.DockStyle.Fill;
            this.grpFiles.Location = new System.Drawing.Point(0, 0);
            this.grpFiles.Name = "grpFiles";
            this.grpFiles.Size = new System.Drawing.Size(470, 430);
            this.grpFiles.TabIndex = 0;
            this.grpFiles.TabStop = false;
            this.grpFiles.Text = "입력 파일";
            // 
            // lstInputFiles
            // 
            this.lstInputFiles.Anchor = ((System.Windows.Forms.AnchorStyles)((((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Bottom)
                        | System.Windows.Forms.AnchorStyles.Left)
                        | System.Windows.Forms.AnchorStyles.Right)));
            this.lstInputFiles.FormattingEnabled = true;
            this.lstInputFiles.ItemHeight = 12;
            this.lstInputFiles.Location = new System.Drawing.Point(14, 22);
            this.lstInputFiles.Name = "lstInputFiles";
            this.lstInputFiles.Size = new System.Drawing.Size(442, 352);
            this.lstInputFiles.TabIndex = 0;
            // 
            // btnAddFile
            // 
            this.btnAddFile.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right)));
            this.btnAddFile.Location = new System.Drawing.Point(300, 389);
            this.btnAddFile.Name = "btnAddFile";
            this.btnAddFile.Size = new System.Drawing.Size(75, 27);
            this.btnAddFile.TabIndex = 1;
            this.btnAddFile.Text = "추가";
            this.btnAddFile.UseVisualStyleBackColor = true;
            this.btnAddFile.Click += new System.EventHandler(this.btnAddFile_Click);
            // 
            // btnRemoveFile
            // 
            this.btnRemoveFile.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Bottom | System.Windows.Forms.AnchorStyles.Right)));
            this.btnRemoveFile.Location = new System.Drawing.Point(381, 389);
            this.btnRemoveFile.Name = "btnRemoveFile";
            this.btnRemoveFile.Size = new System.Drawing.Size(75, 27);
            this.btnRemoveFile.TabIndex = 2;
            this.btnRemoveFile.Text = "제거";
            this.btnRemoveFile.UseVisualStyleBackColor = true;
            this.btnRemoveFile.Click += new System.EventHandler(this.btnRemoveFile_Click);
            // 
            // grpHeader
            // 
            this.grpHeader.Controls.Add(this.lblVersion);
            this.grpHeader.Controls.Add(this.txtVersion);
            this.grpHeader.Controls.Add(this.lblDeviceId);
            this.grpHeader.Controls.Add(this.txtDeviceId);
            this.grpHeader.Controls.Add(this.lblBuildDate);
            this.grpHeader.Controls.Add(this.dtpBuildDate);
            this.grpHeader.Controls.Add(this.lblFlags);
            this.grpHeader.Controls.Add(this.nudFlags);
            this.grpHeader.Controls.Add(this.btnBuild);
            this.grpHeader.Dock = System.Windows.Forms.DockStyle.Fill;
            this.grpHeader.Location = new System.Drawing.Point(0, 0);
            this.grpHeader.Name = "grpHeader";
            this.grpHeader.Size = new System.Drawing.Size(506, 430);
            this.grpHeader.TabIndex = 0;
            this.grpHeader.TabStop = false;
            this.grpHeader.Text = "헤더 정보";
            // 
            // lblVersion
            // 
            this.lblVersion.AutoSize = true;
            this.lblVersion.Location = new System.Drawing.Point(20, 35);
            this.lblVersion.Name = "lblVersion";
            this.lblVersion.Size = new System.Drawing.Size(69, 12);
            this.lblVersion.TabIndex = 0;
            this.lblVersion.Text = "버전(Version)";
            // 
            // txtVersion
            // 
            this.txtVersion.Location = new System.Drawing.Point(130, 31);
            this.txtVersion.Name = "txtVersion";
            this.txtVersion.Size = new System.Drawing.Size(220, 21);
            this.txtVersion.TabIndex = 1;
            this.txtVersion.Text = "1.0.0";
            // 
            // lblDeviceId
            // 
            this.lblDeviceId.AutoSize = true;
            this.lblDeviceId.Location = new System.Drawing.Point(20, 74);
            this.lblDeviceId.Name = "lblDeviceId";
            this.lblDeviceId.Size = new System.Drawing.Size(56, 12);
            this.lblDeviceId.TabIndex = 2;
            this.lblDeviceId.Text = "디바이스 ID";
            // 
            // txtDeviceId
            // 
            this.txtDeviceId.Location = new System.Drawing.Point(130, 70);
            this.txtDeviceId.Name = "txtDeviceId";
            this.txtDeviceId.Size = new System.Drawing.Size(220, 21);
            this.txtDeviceId.TabIndex = 3;
            // 
            // lblBuildDate
            // 
            this.lblBuildDate.AutoSize = true;
            this.lblBuildDate.Location = new System.Drawing.Point(20, 113);
            this.lblBuildDate.Name = "lblBuildDate";
            this.lblBuildDate.Size = new System.Drawing.Size(53, 12);
            this.lblBuildDate.TabIndex = 4;
            this.lblBuildDate.Text = "빌드 날짜";
            // 
            // dtpBuildDate
            // 
            this.dtpBuildDate.Format = System.Windows.Forms.DateTimePickerFormat.Short;
            this.dtpBuildDate.Location = new System.Drawing.Point(130, 109);
            this.dtpBuildDate.Name = "dtpBuildDate";
            this.dtpBuildDate.Size = new System.Drawing.Size(220, 21);
            this.dtpBuildDate.TabIndex = 5;
            // 
            // lblFlags
            // 
            this.lblFlags.AutoSize = true;
            this.lblFlags.Location = new System.Drawing.Point(20, 152);
            this.lblFlags.Name = "lblFlags";
            this.lblFlags.Size = new System.Drawing.Size(34, 12);
            this.lblFlags.TabIndex = 6;
            this.lblFlags.Text = "Flags";
            // 
            // nudFlags
            // 
            this.nudFlags.Location = new System.Drawing.Point(130, 148);
            this.nudFlags.Maximum = new decimal(new int[] {
            65535,
            0,
            0,
            0});
            this.nudFlags.Name = "nudFlags";
            this.nudFlags.Size = new System.Drawing.Size(220, 21);
            this.nudFlags.TabIndex = 7;
            // 
            // btnBuild
            // 
            this.btnBuild.Location = new System.Drawing.Point(130, 193);
            this.btnBuild.Name = "btnBuild";
            this.btnBuild.Size = new System.Drawing.Size(110, 30);
            this.btnBuild.TabIndex = 8;
            this.btnBuild.Text = "빌드 실행";
            this.btnBuild.UseVisualStyleBackColor = true;
            this.btnBuild.Click += new System.EventHandler(this.btnBuild_Click);
            // 
            // txtLog
            // 
            this.txtLog.Dock = System.Windows.Forms.DockStyle.Fill;
            this.txtLog.Location = new System.Drawing.Point(0, 0);
            this.txtLog.Multiline = true;
            this.txtLog.Name = "txtLog";
            this.txtLog.ScrollBars = System.Windows.Forms.ScrollBars.Vertical;
            this.txtLog.Size = new System.Drawing.Size(980, 186);
            this.txtLog.TabIndex = 0;
            // 
            // openFileDialog
            // 
            this.openFileDialog.Filter = "모든 파일|*.*";
            this.openFileDialog.Multiselect = true;
            // 
            // FirmwareBuilderForm
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(7F, 12F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(980, 620);
            this.Controls.Add(this.splitMain);
            this.Name = "FirmwareBuilderForm";
            this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
            this.Text = "Firmware Builder";
            this.splitMain.Panel1.ResumeLayout(false);
            this.splitMain.Panel2.ResumeLayout(false);
            this.splitMain.Panel2.PerformLayout();
            ((System.ComponentModel.ISupportInitialize)(this.splitMain)).EndInit();
            this.splitMain.ResumeLayout(false);
            this.splitTop.Panel1.ResumeLayout(false);
            this.splitTop.Panel2.ResumeLayout(false);
            ((System.ComponentModel.ISupportInitialize)(this.splitTop)).EndInit();
            this.splitTop.ResumeLayout(false);
            this.grpFiles.ResumeLayout(false);
            this.grpHeader.ResumeLayout(false);
            this.grpHeader.PerformLayout();
            ((System.ComponentModel.ISupportInitialize)(this.nudFlags)).EndInit();
            this.ResumeLayout(false);

        }

        #endregion
    }
}
